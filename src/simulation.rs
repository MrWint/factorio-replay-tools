use std::collections::HashSet;

use factorio_serialize::{replay::{Direction, InputAction, InputActionData}, BoundingBox, FixedPoint32_8, MapPosition, TilePosition, Vector};

use crate::prototypes::Prototypes;

pub const PID: u16 = 0;

pub struct GameConfig {
  player_movement_speed: f64,
  player_bounding_box: BoundingBox,
  maximum_corner_sliding_distance: FixedPoint32_8,

  dry_tree_bounding_box: BoundingBox,
}
impl GameConfig {
  fn from_prototypes(prototypes: Prototypes) -> Self {
    GameConfig {
      player_movement_speed: prototypes.character["character"].running_speed,
      player_bounding_box: prototypes.character["character"].collision_box.to_struct(),
      maximum_corner_sliding_distance: FixedPoint32_8::from_double(prototypes.character["character"].maximum_corner_sliding_distance),

      dry_tree_bounding_box: prototypes.tree["dry-tree"].collision_box.to_struct(),
    }
  }
}
lazy_static::lazy_static! {
  static ref GAME_CONFIG: GameConfig = {
    GameConfig::from_prototypes(crate::prototypes::parse_prototype_data())
  };
}

pub struct GameState {
  pub tick: u32,
  player_position: MapPosition,
  player_walking_direction: Direction,

  pub water_tiles: HashSet<TilePosition>,

  pub dry_trees: Vec<MapPosition>,

  instrumented: bool,
  pub input_actions: Vec<InputAction>,
}
impl GameState {
  pub fn new() -> Self {
    Self {
      tick: 0,
      player_position: MapPosition::new(FixedPoint32_8(0), FixedPoint32_8(0)),
      player_walking_direction: Direction::None,

      water_tiles: HashSet::new(),

      dry_trees: Vec::new(),

      instrumented: false,
      input_actions: Vec::new()
    }
  }
  pub fn with_instrumentation(self) -> Self {
    Self { instrumented: true, ..self }
  }
  pub fn make_water_tile(&mut self, position: TilePosition) {
    self.water_tiles.insert(position);
  }
  pub fn add_tree(&mut self, position: MapPosition) {
    self.dry_trees.push(position);
  }
  pub fn set_walking_direction(&mut self, player_walking_direction: Direction) {
    match player_walking_direction {
        Direction::None => self.add_input_action(InputActionData::StopWalking),
        _ => self.add_input_action(InputActionData::StartWalking(player_walking_direction)),
    }
    self.player_walking_direction = player_walking_direction;
  }

  pub fn tick(&mut self) {
    self.tick += 1;

    // Move player
    self.character_update();

    if self.instrumented {
      self.generate_debug_commands();
      self.generate_assert_commands();
    }
  }

  // from Character::update
  fn character_update(&mut self) {
    if self.player_walking_direction != Direction::None {
      let movement_speed = GAME_CONFIG.player_movement_speed;  // ignoring possible modifiers from Character::calculateMovementSpeedAndExtractEnergy
      let movement = Vector::direction_multiplicators(self.player_walking_direction) * movement_speed;
      {  // from Character::changePosition
        if let Some(new_position) = self.calculate_new_position_internal(movement).or_else(|| self.calculate_short_movement(movement)) {
          self.player_position = new_position;
        }
      }
    }
  }

  // from Character::calculateNewPositionInternal
  fn calculate_new_position_internal(&self, movement: Vector) -> Option<MapPosition> {
    if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + movement)) {
      return Some(self.player_position + movement);
    }
    if movement.is_orthogonal() {
      if let Some(new_position) = self.calculate_slide_over_corner(movement) {
        return Some(new_position);
      }
    } else {
      let horizontal_movement = Vector::new(movement.x, 0.0);
      if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + horizontal_movement)) {
        return Some(self.player_position + horizontal_movement);
      }
      let vertical_movement = Vector::new(0.0, movement.y);
      if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + vertical_movement)) {
        return Some(self.player_position + vertical_movement);
      }
    }
    if movement.x * movement.x + movement.y * movement.y <= 0.01 {
      return None;
    }
    return self.calculate_new_position_internal(movement * 0.5)
  }

  // from Character::calculateShortMovement
  fn calculate_short_movement(&self, movement: Vector) -> Option<MapPosition> {
    if !movement.is_orthogonal() || (movement.x == 0.0 && movement.y == 0.0) {
        return None;
    }
    for i in (1..=8).rev() {
      let x = if movement.x == 0.0 { 0 } else { i * if movement.x < 0.0 { -1 } else { 1 }};
      let y = if movement.y == 0.0 { 0 } else { i * if movement.y < 0.0 { -1 } else { 1 }};
      let offset = MapPosition::new(FixedPoint32_8(x), FixedPoint32_8(y));
      if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + offset)) {
        return Some(self.player_position + offset);
      }
    }
    None
  }

  // from Character::checkCollisionDependingOnLatencyHidingMode, Surface::collide
  fn check_collision(&self, player_bounding_box: &BoundingBox) -> bool {
    self.const_collide_with_tile(player_bounding_box).is_some() || self.collide_with_entity(player_bounding_box).is_some()
  }
  // Surface::constCollideWithTile, collideWithTileWithTransitionsCommon__lambda
  fn const_collide_with_tile(&self, player_bounding_box: &BoundingBox) -> Option<Direction> {
    let new_position = MapPosition::new(FixedPoint32_8((player_bounding_box.right_bottom.x.0 + player_bounding_box.left_top.x.0) / 2), FixedPoint32_8((player_bounding_box.right_bottom.y.0 + player_bounding_box.left_top.y.0) / 2));
    let tile_position = new_position.to_tile_position();
    let position_in_tile = new_position - tile_position.top_left_map_position();
    if !self.water_tiles.contains(&tile_position) { return None; }
    let mut water_mask: u8 = 0;
    if self.water_tiles.contains(&TilePosition { x: tile_position.x, y: tile_position.y - 1 }) { water_mask |= 1 };
    if self.water_tiles.contains(&TilePosition { x: tile_position.x + 1, y: tile_position.y }) { water_mask |= 2 };
    if self.water_tiles.contains(&TilePosition { x: tile_position.x, y: tile_position.y + 1 }) { water_mask |= 4 };
    if self.water_tiles.contains(&TilePosition { x: tile_position.x - 1, y: tile_position.y }) { water_mask |= 8 };
    match water_mask {
      0b_0111 | 0b_1011 | 0b_1101 | 0b_1110 | 0b_1111 => Some(Direction::None),
      0b_0011 => if self.water_tiles.contains(&TilePosition { x: tile_position.x + 1, y: tile_position.y - 1 }) && position_in_tile.y < position_in_tile.x {
        Some(Direction::NorthEast)
      } else { None },
      0b_0110 => if self.water_tiles.contains(&TilePosition { x: tile_position.x + 1, y: tile_position.y + 1 }) && (256 - position_in_tile.y.0) < position_in_tile.x.0 {
        Some(Direction::SouthEast)
      } else { None },
      0b_1100 => if self.water_tiles.contains(&TilePosition { x: tile_position.x - 1, y: tile_position.y + 1 }) && position_in_tile.y > position_in_tile.x {
        Some(Direction::SouthWest)
      } else { None },
      0b_1001 => if self.water_tiles.contains(&TilePosition { x: tile_position.x - 1, y: tile_position.y - 1 }) && (256 - position_in_tile.y.0) > position_in_tile.x.0 {
        Some(Direction::NorthWest)
      } else { None },
      _ => None,
    }
  }
  // from Surface::collideWithEntity (loosely)
  // note: which entity is returned in case there are multiple overlaps depends on the order in which the entities are stored in the game
  fn collide_with_entity(&self, player_bounding_box: &BoundingBox) -> Option<BoundingBox> {
    for bounding_box in self.all_entity_collision_boxes() {
      if player_bounding_box.collide_bounding_box(&bounding_box) { return Some(bounding_box.clone()); }
    }
    None
  }
  fn all_entity_collision_boxes(&self) -> impl IntoIterator<Item = BoundingBox> {
    // TODO: add entities to check collisions for
    self.dry_trees.iter().map(|&mp| GAME_CONFIG.dry_tree_bounding_box.offset(mp)).collect::<Vec<_>>()
  }
  // from Character::calculateSlideOverCorner
  fn calculate_slide_over_corner(&self, movement: Vector) -> Option<MapPosition> {
    let new_position_bounding_box = GAME_CONFIG.player_bounding_box.offset(self.player_position + movement);
    if let Some(bounding_box) = self.collide_with_entity(&new_position_bounding_box) {
      if movement.x == 0.0 {
        self.calculate_vertical_slide(movement, &bounding_box)
      } else {
        self.calculate_horizontal_slide(movement, &bounding_box)
      }
    } else { // from Character::calculateTileSlide
      match self.const_collide_with_tile(&new_position_bounding_box) {
        None => None,
        Some(Direction::None) => if movement.x == 0.0 {
          self.calculate_vertical_slide(movement, &BoundingBox::tile_box(self.player_position.to_tile_position(), 0.0))
        } else {
          self.calculate_horizontal_slide(movement, &BoundingBox::tile_box(self.player_position.to_tile_position(), 0.0))
        },
        Some(direction) => {
          let direction_multiplicator = Vector::direction_multiplicators(direction);
          let correction_factor = -(movement.x * direction_multiplicator.x + movement.y * direction_multiplicator.y);
          let new_movement = movement + direction_multiplicator * correction_factor;
          if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + new_movement)) {
            Some(self.player_position + new_movement)
          } else { None }
        },
      }
    }
  }
  // from Character::calculateVerticalSlide
  fn calculate_vertical_slide(&self, movement: Vector, colliding_bounding_box: &BoundingBox) -> Option<MapPosition> {
    let test_bounding_box = GAME_CONFIG.player_bounding_box.offset(self.player_position + movement);
    let right_nudge_distance = colliding_bounding_box.right_bottom.x - test_bounding_box.left_top.x;
    let left_nudge_distance = test_bounding_box.right_bottom.x - colliding_bounding_box.left_top.x;
    if std::cmp::min(left_nudge_distance, right_nudge_distance) > GAME_CONFIG.maximum_corner_sliding_distance { return None; }
    let slide_margin = FixedPoint32_8::from_double(0.01);
    let nudge_movement = if right_nudge_distance >= left_nudge_distance {
      let goal_x_pos = colliding_bounding_box.left_top.x - GAME_CONFIG.player_bounding_box.right_bottom.x - slide_margin;
      if self.check_collision(&GAME_CONFIG.player_bounding_box.offset(MapPosition::new(goal_x_pos, self.player_position.y) + movement)) {
        return None;
      }
      Vector::new(-movement.y.abs().min(left_nudge_distance.0 as f64 * (1.0 / 256.0) + 0.01), 0.0)
    } else {
      let goal_x_pos = colliding_bounding_box.right_bottom.x - GAME_CONFIG.player_bounding_box.left_top.x + slide_margin;
      if self.check_collision(&GAME_CONFIG.player_bounding_box.offset(MapPosition::new(goal_x_pos, self.player_position.y) + movement)) {
        return None;
      }
      Vector::new(movement.y.abs().min(right_nudge_distance.0 as f64 * (1.0 / 256.0) + 0.01), 0.0)
    };
    if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + nudge_movement)) {
      Some(self.player_position + nudge_movement)
    } else { None }
  }
  // from Character::calculateHorizontalSlide
  fn calculate_horizontal_slide(&self, movement: Vector, colliding_bounding_box: &BoundingBox) -> Option<MapPosition> {
    let test_bounding_box = GAME_CONFIG.player_bounding_box.offset(self.player_position + movement);
    let down_nudge_distance = colliding_bounding_box.right_bottom.y - test_bounding_box.left_top.y;
    let up_nudge_distance = test_bounding_box.right_bottom.y - colliding_bounding_box.left_top.y;
    if std::cmp::min(up_nudge_distance, down_nudge_distance) > GAME_CONFIG.maximum_corner_sliding_distance { return None; }
    let slide_margin = FixedPoint32_8::from_double(0.01);
    let nudge_movement = if up_nudge_distance >= down_nudge_distance  {
      let goal_y_pos = colliding_bounding_box.right_bottom.y - GAME_CONFIG.player_bounding_box.left_top.y + slide_margin;
      if self.check_collision(&GAME_CONFIG.player_bounding_box.offset(MapPosition::new(self.player_position.x, goal_y_pos) + movement)) {
        return None;
      }
      Vector::new(0.0, movement.x.abs().min(down_nudge_distance.0 as f64 * (1.0 / 256.0) + 0.01))
    } else {
      let goal_y_pos = colliding_bounding_box.left_top.y - GAME_CONFIG.player_bounding_box.right_bottom.y - slide_margin;
      if self.check_collision(&GAME_CONFIG.player_bounding_box.offset(MapPosition::new(self.player_position.x, goal_y_pos) + movement)) {
        return None;
      }
      Vector::new(0.0, -movement.x.abs().min(up_nudge_distance.0 as f64 * (1.0 / 256.0) + 0.01))
    };
    if !self.check_collision(&GAME_CONFIG.player_bounding_box.offset(self.player_position + nudge_movement)) {
      Some(self.player_position + nudge_movement)
    } else { None }
  }

  fn generate_debug_commands(&mut self) {
    // print tick
    // self.run_command(format!(r#"game.write_file("{DEBUG_FILE}", "tick expected {}, actual " .. game.tick .. "\n", true)"#, self.tick));

    // print position
    // self.run_command(format!(r#"ppp()"#));
  }

  fn generate_assert_commands(&mut self) {
    self.run_command(format!(r#"assert_tick({})"#, self.tick));
    self.run_command(format!(r#"assert_player_position({}, {})"#, self.player_position.x.0, self.player_position.y.0));
  }

  fn run_command<S: AsRef<str>>(&mut self, command: S) {
    self.add_input_action(InputActionData::WriteToConsole(format!("/sc {}", command.as_ref())))
  }
  fn add_input_action(&mut self, action: InputActionData) {
    self.input_actions.push(InputAction::new(self.tick, PID, action))
  }

}