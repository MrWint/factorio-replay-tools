use std::collections::{HashMap, HashSet, VecDeque};

use factorio_serialize::{constants::{Item, Recipe}, replay::{BuildParameters, CraftData, Direction, InputAction, InputActionData, ItemStackTargetSpecification, QuickBarPickSlotParameters, QuickBarSetSlotParameters, SetFilterParameters}, BoundingBox, FixedPoint32_8, MapPosition, RandomGenerator, TilePosition, Vector};

use crate::{gameconfig::{ProductConfig, GAME_CONFIG}, hexfloat::HexFloat};

pub const PID: u16 = 0;
pub fn num_ticks_until(goal: f64, speed: f64) -> u32 {
  let step = speed / 60.0;
  let mut current = 0.0;
  let mut ticks = 0;
  while current < goal {
    ticks += 1;
    current += step;
  }
  return ticks;
}

struct CraftingOrder {
  recipe: Recipe,
  count: u32,
  current_energy: f64,
}
struct Burner {
  fuel_slot: Option<(Item, u32)>,
  remaining_part_of_burning_fuel: f64,
  heat_capacity: f64,
  heat_energy: f64,
}
impl Burner {
  fn with_buffer_size(heat_capacity: f64) -> Self {
    Burner { fuel_slot: None, remaining_part_of_burning_fuel: 0.0, heat_capacity, heat_energy: 0.0 }
  }
  // from Burner::extractEnergyAndPollute
  fn extract_energy(&mut self, desired_energy: f64) -> f64 {
    let provided_energy = desired_energy.min(self.heat_energy);
    self.heat_energy -= provided_energy;
    provided_energy
  }
  // from Burner::transferHeat
  fn transfer_heat(&mut self) {
    let mut heat_refill_amount = self.heat_capacity - self.heat_energy;
    if heat_refill_amount > self.remaining_part_of_burning_fuel {
      heat_refill_amount = self.remaining_part_of_burning_fuel;
    }
    if heat_refill_amount != 0.0 {
      self.heat_energy = self.heat_capacity.min(self.heat_energy + heat_refill_amount);
      self.remaining_part_of_burning_fuel = 0f64.max(self.remaining_part_of_burning_fuel - heat_refill_amount);
    }
  }
  // from Burner::update
  fn tick(&mut self) {
    if self.remaining_part_of_burning_fuel > 0.0 && self.heat_energy < self.heat_capacity {
      self.transfer_heat();
    }
    if self.heat_energy < self.heat_capacity && self.remaining_part_of_burning_fuel <= 0.0 {
      if let Some((item, amount)) = self.fuel_slot {
        self.remaining_part_of_burning_fuel = GAME_CONFIG.fuels[&item].fuel_value;
        self.fuel_slot = if amount > 1 { Some((item, amount - 1)) } else { None };
        self.transfer_heat()
      }      
    }
  }
}
pub struct BurnerMiner {
  energy_source: Burner,
  mining_progress: f64,
  num_resources_mined: u32,
}
impl BurnerMiner {
  fn new() -> Self {
    BurnerMiner {
      energy_source: Burner::with_buffer_size(GAME_CONFIG.burner_miner_energy_usage * (16.0 / 15.0)), // from MiningDrill::onEffectChanged
      mining_progress: 0.0,
      num_resources_mined: 0,
    }
  }
  pub fn num_resources_needed_on_map(&self) -> u32 {
    self.num_resources_mined + 1
  }
  // from MiningDrill::update
  fn tick(&mut self) -> bool {  // true if resource mined this tick
    // assuming there is space for the resulting item
    let energy_consumed = self.energy_source.extract_energy(GAME_CONFIG.burner_miner_energy_usage);
    if energy_consumed != 0.0 {
      let energy_satisfaction = energy_consumed / GAME_CONFIG.burner_miner_energy_usage;
      self.mining_progress += (energy_satisfaction * GAME_CONFIG.burner_miner_speed) * (1.0 / 60.0);
    }
    self.energy_source.tick();
    // from MiningDrill::performMining
    const MINING_TIME: f64 = 1.0; // hard-coded: same value for all basic resources
    if self.mining_progress >= MINING_TIME {
      self.mining_progress -= MINING_TIME;
      self.num_resources_mined += 1;
      return true;
    }
    false
  }
}
#[derive(Debug)]
enum PlayerSelectedEntity {
  DryTree(usize),
  HugeRock(usize),
  IronOre(TilePosition),
  CopperOre(TilePosition),
}
pub struct GameState {
  pub tick: u32,

  player_position: MapPosition,
  player_walking_direction: Direction,
  player_selected_entity: Option<PlayerSelectedEntity>,
  player_mining_progress: Option<(f64, f64, u32)>,
  player_inventory: HashMap<Item, u32>,
  player_crafting_queue: VecDeque<CraftingOrder>,

  pub water_tiles: HashSet<TilePosition>,

  dry_trees: Vec<MapPosition>,
  huge_rocks: Vec<MapPosition>,
  huge_rock_rng: RandomGenerator,
  pub iron_ores: HashMap<TilePosition, u32>,
  pub copper_ores: HashMap<TilePosition, u32>,
  pub iron_miners: HashMap<TilePosition, BurnerMiner>,

  instrumented: bool,
  pub input_actions: Vec<InputAction>,
}
impl GameState {
  pub fn new(huge_rock_rng: RandomGenerator) -> Self {
    Self {
      tick: 0,
      player_position: MapPosition::new(FixedPoint32_8(0), FixedPoint32_8(0)),
      player_walking_direction: Direction::None,
      player_selected_entity: None,
      player_mining_progress: None,
      player_inventory: [(Item::IronPlate, 8), (Item::Wood, 1), (Item::BurnerMiningDrill, 1), (Item::StoneFurnace, 1)].into_iter().collect(),
      player_crafting_queue: VecDeque::new(),

      water_tiles: HashSet::new(),

      dry_trees: Vec::new(),
      huge_rocks: Vec::new(),
      huge_rock_rng,
      iron_ores: HashMap::new(),
      copper_ores: HashMap::new(),
      iron_miners: HashMap::new(),

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
  fn pick_tree_at_position(&self, position: MapPosition) -> Option<usize> {
    let bb = GAME_CONFIG.dry_tree_bounding_box.with_direction(Direction::South).offset(position); // get inverse bounding box for check
    self.dry_trees.iter().position(|p| bb.collide_point(&p))
  }
  pub fn mine_tree(&mut self, position: MapPosition) -> u32 {
    assert!(self.player_mining_progress.is_none(), "player is already mining {:?} progress {:?}", self.player_selected_entity, self.player_mining_progress);
    let tree_index = self.pick_tree_at_position(position).expect("cannot find tree at position");
    self.player_selected_entity = Some(PlayerSelectedEntity::DryTree(tree_index));
    self.player_mining_progress = Some((0.0, GAME_CONFIG.dry_tree_mining_time, 1));

    self.add_input_action(InputActionData::SelectedEntityChanged(position)); // Select entity
    self.add_input_action(InputActionData::BeginMining); // Begin mining
    num_ticks_until(GAME_CONFIG.dry_tree_mining_time, GAME_CONFIG.player_mining_speed)
  }
  pub fn add_rock(&mut self, position: MapPosition) {
    self.huge_rocks.push(position);
  }
  fn pick_rock_at_position(&self, position: MapPosition) -> Option<usize> {
    let bb = GAME_CONFIG.huge_rock_bounding_box.with_direction(Direction::South).offset(position); // get inverse bounding box for check
    self.huge_rocks.iter().position(|p| bb.collide_point(&p))
  }
  pub fn mine_rock(&mut self, position: MapPosition) -> u32 {
    assert!(self.player_mining_progress.is_none(), "player is already mining {:?} progress {:?}", self.player_selected_entity, self.player_mining_progress);
    let rock_index = self.pick_rock_at_position(position).expect("cannot find rock at position");
    self.player_selected_entity = Some(PlayerSelectedEntity::HugeRock(rock_index));
    self.player_mining_progress = Some((0.0, GAME_CONFIG.huge_rock_mining_time, 1));

    self.add_input_action(InputActionData::SelectedEntityChanged(position)); // Select entity
    self.add_input_action(InputActionData::BeginMining); // Begin mining
    num_ticks_until(GAME_CONFIG.huge_rock_mining_time, GAME_CONFIG.player_mining_speed)
  }
  pub fn mine_iron_ore(&mut self, position: TilePosition, count: u32) -> u32 {
    assert!(self.player_mining_progress.is_none(), "player is already mining {:?} progress {:?}", self.player_selected_entity, self.player_mining_progress);
    self.iron_ores.entry(position).or_insert(0);

    self.player_selected_entity = Some(PlayerSelectedEntity::IronOre(position));
    self.player_mining_progress = Some((0.0, GAME_CONFIG.iron_ore_mining_time, count));

    self.add_input_action(InputActionData::SelectedEntityChanged(position.center_map_position())); // Select entity
    self.add_input_action(InputActionData::BeginMining); // Begin mining
    num_ticks_until(GAME_CONFIG.iron_ore_mining_time, GAME_CONFIG.player_mining_speed) * count
  }
  pub fn mine_copper_ore(&mut self, position: TilePosition, count: u32) -> u32 {
    assert!(self.player_mining_progress.is_none(), "player is already mining {:?} progress {:?}", self.player_selected_entity, self.player_mining_progress);
    self.copper_ores.entry(position).or_insert(0);

    self.player_selected_entity = Some(PlayerSelectedEntity::CopperOre(position));
    self.player_mining_progress = Some((0.0, GAME_CONFIG.copper_ore_mining_time, count));

    self.add_input_action(InputActionData::SelectedEntityChanged(position.center_map_position())); // Select entity
    self.add_input_action(InputActionData::BeginMining); // Begin mining
    num_ticks_until(GAME_CONFIG.iron_ore_mining_time, GAME_CONFIG.player_mining_speed) * count
  }
  pub fn set_walking_direction(&mut self, player_walking_direction: Direction) {
    match player_walking_direction {
        Direction::None => self.add_input_action(InputActionData::StopWalking),
        _ => self.add_input_action(InputActionData::StartWalking(player_walking_direction)),
    }
    self.player_walking_direction = player_walking_direction;
  }
  pub fn craft(&mut self, recipe: Recipe, count: u32) -> u32 {
    for ingredient in &GAME_CONFIG.recipes[&recipe].ingredients {
      match ingredient {
        &ProductConfig::Item { id, amount } => self.remove_from_inventory(id, amount * count),
        ProductConfig::Fluid { .. } => panic!("recipe {recipe:?} contains fluid input {ingredient:?} and can't be hand crafted")
      }
    }
    if self.player_crafting_queue.back().is_some_and(|c| c.recipe == recipe) {
      self.player_crafting_queue.back_mut().unwrap().count += count;
    } else {
      self.player_crafting_queue.push_back(CraftingOrder { recipe, count, current_energy: 0.0 });
    }

    self.add_input_action(InputActionData::Craft(CraftData { recipe, count })); // begin crafting
    num_ticks_until(GAME_CONFIG.recipes[&recipe].energy_required, 1.0) * count
  }
  fn remove_from_inventory(&mut self, item: Item, count: u32) {
    let inventory_count = self.player_inventory.get_mut(&item).unwrap_or_else(|| panic!("no {item:?} in inventory to remove"));
    assert!(*inventory_count >= count, "not enough {item:?} in inventory: needed {} but found {}", count, *inventory_count);
    *inventory_count -= count;
    if *inventory_count == 0 {
      self.player_inventory.remove(&item);
    }
  }
  fn add_to_inventory(&mut self, item: Item, count: u32) {
    *self.player_inventory.entry(item).or_insert(0) += count;

    assert!(self.inventory_slots_used() <= GAME_CONFIG.player_inventory_size, "inventory overflow after adding {item:?} x{count}");
  }
  fn inventory_slots_used(&self) -> u32 {
    self.player_inventory.iter().map(|(item, count)| count.div_ceil(GAME_CONFIG.items[item].stack_size)).sum()
  }
  pub fn build_stone_furnace(&mut self, position: TilePosition) {
    self.build(Item::StoneFurnace, position.top_left_map_position(), Direction::North);
  }
  pub fn build_iron_miner(&mut self, position: TilePosition, direction: Direction) {
    self.iron_miners.insert(position, BurnerMiner::new());
    self.build(Item::BurnerMiningDrill, position.top_left_map_position(), direction);
  }
  fn build(&mut self, item: Item, position: MapPosition, direction: Direction) {
    self.remove_from_inventory(item, 1);
    self.add_input_action(InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, })); // Configure quickbar slot
    self.add_input_action(InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, })); // Select quickbar into cursor
    self.add_input_action(InputActionData::Build(BuildParameters { position, direction, created_by_moving: false, build_by_moving_start_position: None, flags: 0, })); // Build item from cursor
    self.add_input_action(InputActionData::ClearCursor); // Clear cursor
    self.add_input_action(InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 })); // Clear quickbar slot
  }
  pub fn add_fuel_to_iron_miner(&mut self, item: Item, amount: u32, position: TilePosition) {
    self.drop_item_at(item, amount, position.top_left_map_position());
    if let Some((i, a)) = &mut self.iron_miners.get_mut(&position).unwrap_or_else(|| panic!("burner at position {position:?} not found")).energy_source.fuel_slot {
      assert!(*i == item, "burner at {position:?} already has incompatible fuel {i:?}, can't add {item:?}");
      *a += amount;
    } else {
      self.iron_miners.get_mut(&position).unwrap().energy_source.fuel_slot = Some((item, amount));
    }
  }
  pub fn drop_item_at(&mut self, item: Item, amount: u32, position: MapPosition) {
    self.remove_from_inventory(item, amount);
    self.add_input_action(InputActionData::SetFilter(SetFilterParameters { target: ItemStackTargetSpecification::from_quick_bar(0, 0), filter: item, })); // Configure quickbar slot
    self.add_input_action(InputActionData::QuickBarPickSlot(QuickBarPickSlotParameters { location: 0, pick_ghost_cursor: false, cursor_split: false, })); // Select quickbar into cursor
    self.add_input_action(InputActionData::SelectedEntityChanged(position)); // Select entity
    for _ in 0..amount {
      self.add_input_action(InputActionData::DropItem(position));
    }
    self.add_input_action(InputActionData::SelectedEntityCleared); // Clear selection
    self.add_input_action(InputActionData::ClearCursor); // Clear cursor
    self.add_input_action(InputActionData::QuickBarSetSlot(QuickBarSetSlotParameters { target_quick_bar_slot: 0, item_to_use: ItemStackTargetSpecification::from_nothing(), currently_selected_quick_bar_slot: 65535 })); // Clear quickbar slot
  }


  pub fn tick(&mut self) {
    self.tick += 1;

    // Move player
    self.character_update();

    // Mining update
    if let Some((mut current, goal, count)) = self.player_mining_progress {
      current += GAME_CONFIG.player_mining_speed / 60.0;
      if current <= goal {
        self.player_mining_progress = Some((current, goal, count))
      } else {
        match &self.player_selected_entity {
          &Some(PlayerSelectedEntity::DryTree(index)) => {
            self.dry_trees.swap_remove(index);
            self.add_to_inventory(Item::Wood, 4);
          },
          &Some(PlayerSelectedEntity::HugeRock(index)) => {
            self.huge_rocks.swap_remove(index);
            let (stone, coal) = self.huge_rock_rng.get_huge_rock_items();
            self.add_to_inventory(Item::Stone, stone);
            self.add_to_inventory(Item::Coal, coal);
          },
          &Some(PlayerSelectedEntity::IronOre(position)) => {
            *self.iron_ores.get_mut(&position).unwrap() += 1;
            self.add_to_inventory(Item::IronOre, 1);
          },
          &Some(PlayerSelectedEntity::CopperOre(position)) => {
            *self.copper_ores.get_mut(&position).unwrap() += 1;
            self.add_to_inventory(Item::CopperOre, 1);
          },
          e => panic!("mining completed of unknown entity {e:?}")
        }
        if count > 1 {
          self.player_mining_progress = Some((0.0, goal, count - 1))
        } else {
          self.player_selected_entity = None;
          self.player_mining_progress = None;
          self.add_input_action(InputActionData::StopMining);
          self.add_input_action(InputActionData::SelectedEntityCleared);
        }
      }
    }

    // Crafting update
    if let Some(mut order) = self.player_crafting_queue.pop_front() {
      order.current_energy += 1.0 / 60.0;
      if order.current_energy < GAME_CONFIG.recipes[&order.recipe].energy_required {
        self.player_crafting_queue.push_front(order);
      } else {
        for result in &GAME_CONFIG.recipes[&order.recipe].results {
          match result {
            &ProductConfig::Item { id, amount } => self.add_to_inventory(id, amount),
            &ProductConfig::Fluid { .. } => panic!("hand-crafted recipe {:?} contains fluid result {result:?}", order.recipe),
          }
        }
        if order.count > 1 {
          order.count -= 1;
          order.current_energy = 0.0;
          self.player_crafting_queue.push_front(order);
        }
      }
    }

    // Miner update
    for (_, miner) in self.iron_miners.iter_mut() {
      miner.tick();
    }

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
    if let Some((current, goal, _)) = self.player_mining_progress {
      self.run_command(format!(r#"assert_player_mining_progress({})"#, HexFloat(current / goal)));
    }
    self.run_command(format!(r#"assert_player_crafting_queue_size({})"#, self.player_crafting_queue.len()));
    if let Some(order) = self.player_crafting_queue.front() {
      self.run_command(format!(r#"assert_player_crafting_progress({})"#, HexFloat(order.current_energy / GAME_CONFIG.recipes[&order.recipe].energy_required)));
    }
    for (item, count) in self.player_inventory.clone() {
      self.run_command(format!(r#"assert_player_inventory_item_count("{}", {})"#, item.name(), count));
    }

    for (position, miner) in self.iron_miners.iter() {
      self.input_actions.push(self.build_command(format!(r#"assert_miner_mining_progress({}, {}, {})"#, position.x, position.y, HexFloat(miner.mining_progress))));
      self.input_actions.push(self.build_command(format!(r#"assert_miner_remaining_burning_fuel({}, {}, {})"#, position.x, position.y, HexFloat(miner.energy_source.remaining_part_of_burning_fuel))));
      self.input_actions.push(self.build_command(format!(r#"assert_miner_heat({}, {}, {})"#, position.x, position.y, HexFloat(miner.energy_source.heat_energy))));
    }
  }

  fn build_command<S: AsRef<str>>(&self, command: S) -> InputAction {
    InputAction::new(self.tick, PID, InputActionData::WriteToConsole(format!("/sc {}", command.as_ref())))
  }
  fn run_command<S: AsRef<str>>(&mut self, command: S) {
    self.add_input_action(InputActionData::WriteToConsole(format!("/sc {}", command.as_ref())))
  }
  fn add_input_action(&mut self, action: InputActionData) {
    self.input_actions.push(InputAction::new(self.tick, PID, action))
  }

}