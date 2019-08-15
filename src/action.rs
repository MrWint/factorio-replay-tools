use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::constants::*;
use crate::replay::*;
use num_traits::{FromPrimitive, ToPrimitive};
#[allow(unused_imports)] use std::io::{Error, ErrorKind, Read, Result, Write};
#[derive(Debug)]
pub struct Slot { // ItemStackTargetSpecification
  pub typ: SlotType,
  pub slot: u16,
}
impl Slot {
  fn read(r: &mut ReplayReader) -> Result<Self> {
    let inventory_id = r.read_u8()?;
    let slot = r.read_u16::<LittleEndian>()?;
    let inventory_type = r.read_u8()?;
    let inventory_type_id = u16::from(inventory_type) << 8 | u16::from(inventory_id);
    if let Some(typ) = SlotType::from_u16(inventory_type_id) {
      r.read_u8_assert(0)?;
      Ok(Slot { typ, slot, })
    } else {
      Err(Error::new(ErrorKind::NotFound, format!("Unknown inventory_type_id {:#x} (slot {}) at position {}", inventory_type_id, slot, r.position() - 4)))
    }
  }
  pub fn write(&self, w: &mut ReplayWriter) -> Result<()> {
    let inventory_type_id = self.typ.to_u16().unwrap();
    w.write_u8(inventory_type_id as u8)?;
    w.write_u16::<LittleEndian>(self.slot)?;
    w.write_u8((inventory_type_id >> 8) as u8)?;
    w.write_u8(0)
  }
}

#[derive(Debug)]
pub enum InputAction {
  BeginMining,
  BuildItem { x: i32, y: i32, dir: CardinalDirection, ghost: bool, },
  ChangeActiveItemGroupForCrafting { item_group: u8, },
  CleanCursorStack,
  CloseGui,
  Craft { recipe: Recipe, amount: u32, },
  CursorSplit { slot: Slot, },
  CursorTransfer { slot: Slot, },
  FastEntityTransfer { dir: TransferDirection, },
  FastEntitySplit { dir: TransferDirection, },
  GameCreatedFromScenario,
  InventoryTransfer { slot: Slot, },
  InventorySplit { slot: Slot, },
  OpenCharacterGui,
  OpenGui,
  OpenTechnologyGui,
  PlayerJoinGame { player_id: u16, name: String },
  QuickBarPickSlot { slot: u16, },
  QuickBarSetSlot { slot: u16,  source_slot: Slot, },
  SelectedEntityChanged { x: i32, y: i32, },
  SelectedEntityChangedRelative { x: i16, y: i16, },
  SelectedEntityChangedVeryClose { x: i8, y: i8, },
  SelectedEntityChangedVeryClosePrecise { x: i8, y: i8, },
  SelectedEntityCleared,
  SetFilter { slot: Slot, item: Item, },
  SingleplayerInit,
  StackSplit { slot: Slot, },
  StackTransfer { slot: Slot, },
  StartResearch { technology: u16 },
  StartWalking { dir: Direction },
  StopBuildingByMoving,
  StopMining,
  StopWalking,
  ToggleShowEntityInfo,
}
impl InputAction {
  pub fn read(action_type: InputActionType, action_type_pos: u64, r: &mut ReplayReader) -> Result<Option<InputAction>> {
    match action_type {
      InputActionType::QuickBarSetSelectedPage => {
        let bar = r.read_u8()?; // top or bottom quickbar
        let set = r.read_u8()?; // 0-9
        println!("ignoring QuickBarSetSelectedPage setting bar {} to set {}!", bar, set);
        Ok(None)
      }
      InputActionType::ChangeActiveItemGroupForFilters => {
        let item_group = r.read_u8()?;
        println!("ignoring ChangeActiveItemGroupForFilters group {}!", item_group);
        Ok(None)
      },
      InputActionType::ChangePickingState => {
        let val1 = r.read_u8()?;
        println!("ignoring ChangePickingState {}!", val1);
        Ok(None)
      },
      InputActionType::QuickBarSetSlot => {
        let slot = r.read_u16::<LittleEndian>()?; // fixed point number
        let source_slot = Slot::read(r)?;
        Ok(Some(InputAction::QuickBarSetSlot { slot,  source_slot, }))
      },
      InputActionType::SmartPipette => {
        let fixed1 = r.read_u16::<LittleEndian>()?;
        let allowed_commands2 = r.read_u8()?;
        let val3 = r.read_u8()?;
        println!("ignoring SmartPipette fixed1 {:#x} allowed_commands2 {} val3 {}!", fixed1, allowed_commands2, val3);
        Ok(None)
      },
      InputActionType::BeginMining => Ok(Some(InputAction::BeginMining)),
      InputActionType::BuildItem => {
        let x = r.read_i32::<LittleEndian>()?;
        let y = r.read_i32::<LittleEndian>()?;
        let dir = CardinalDirection::from_u8(r.read_u8()?).unwrap();
        let _is_dragging = r.read_u8()?; // 0 = dragging? Doesn't seem to affect things.
        r.read_u8_assert(1)?;
        let ghost = r.read_u8()? != 0;
        r.read_u8_assert(0)?; // unknown boolean
        Ok(Some(InputAction::BuildItem { x, y, dir, ghost, }))
      },
      InputActionType::ChangeActiveItemGroupForCrafting => {  Ok(Some(InputAction::ChangeActiveItemGroupForCrafting { item_group: r.read_u8()?, })) },
      InputActionType::CheckCRC => {
        r.read_u32::<LittleEndian>()?;
        r.read_u32::<LittleEndian>()?;
        Ok(None)
      },
      InputActionType::CheckCRCHeuristic => {
        r.read_u32::<LittleEndian>()?;
        r.read_u32::<LittleEndian>()?;
        Ok(None)
      },
      InputActionType::CleanCursorStack => Ok(Some(InputAction::CleanCursorStack)),
      InputActionType::CloseGui => Ok(Some(InputAction::CloseGui)),
      InputActionType::Craft => {
        let recipe = Recipe::from_u16(r.read_u16::<LittleEndian>()?).unwrap();
        let amount = r.read_u32::<LittleEndian>()?;
        Ok(Some(InputAction::Craft { recipe, amount, }))
      },
      InputActionType::CursorSplit => {  Ok(Some(InputAction::CursorSplit { slot: Slot::read(r)? })) },
      InputActionType::CursorTransfer => {  Ok(Some(InputAction::CursorTransfer { slot: Slot::read(r)? })) },
      InputActionType::DisplayResolutionChanged => {
        let _x = r.read_u32::<LittleEndian>()?; // ChunkPosition
        let _y = r.read_u32::<LittleEndian>()?;
        println!("ignoring DisplayResolutionChanged!");
        Ok(None)
      },
      InputActionType::FastEntitySplit => {  Ok(Some(InputAction::FastEntitySplit { dir: TransferDirection::from_u8(r.read_u8()?).unwrap() })) },
      InputActionType::FastEntityTransfer => {  Ok(Some(InputAction::FastEntityTransfer { dir: TransferDirection::from_u8(r.read_u8()?).unwrap() })) },
      InputActionType::GameCreatedFromScenario => Ok(Some(InputAction::GameCreatedFromScenario)),
      InputActionType::InventorySplit => {  Ok(Some(InputAction::InventorySplit { slot: Slot::read(r)? })) },
      InputActionType::InventoryTransfer => {  Ok(Some(InputAction::InventoryTransfer { slot: Slot::read(r)? })) },
      InputActionType::OpenCharacterGui => Ok(Some(InputAction::OpenCharacterGui)),
      InputActionType::OpenGui => Ok(Some(InputAction::OpenGui)),
      InputActionType::OpenTechnologyGui => Ok(Some(InputAction::OpenTechnologyGui)),
      InputActionType::PlayerJoinGame => {
        let player_id = r.read_opt_u16()?;
        r.read_u16_assert(0)?;
        r.read_u8_assert(1)?; // AllowedCommands
        let name = r.read_string()?;
        r.read_u8_assert(0)?;
        r.read_u8_assert(1)?;
        Ok(Some(InputAction::PlayerJoinGame { player_id, name }))
      },
      InputActionType::QuickBarPickSlot => {
        let slot = r.read_u16::<LittleEndian>()?;
        r.read_u8_assert(0)?; // unknown boolean
        r.read_u8_assert(0)?; // unknown boolean
        Ok(Some(InputAction::QuickBarPickSlot { slot, }))
      },
      InputActionType::SelectedEntityChanged => {
        let x = r.read_i32::<LittleEndian>()?;
        let y = r.read_i32::<LittleEndian>()?;
        Ok(Some(InputAction::SelectedEntityChanged { x, y, }))
      },
      InputActionType::SelectedEntityChangedRelative => {
        let y = r.read_i16::<LittleEndian>()?;
        let x = r.read_i16::<LittleEndian>()?;
        Ok(Some(InputAction::SelectedEntityChangedRelative { x, y, }))
      },
      InputActionType::SelectedEntityChangedVeryClose => {
        let xy = r.read_u8()?;
        let x = (xy >> 4) as i8 - 8;
        let y = (xy & 0x0f) as i8 - 8;
        Ok(Some(InputAction::SelectedEntityChangedVeryClose { x, y, }))
      },
      InputActionType::SelectedEntityChangedVeryClosePrecise => {
        let y = r.read_i8()?;
        let x = r.read_i8()?;
        Ok(Some(InputAction::SelectedEntityChangedVeryClosePrecise { x, y, }))
      },
      InputActionType::SelectedEntityCleared => Ok(Some(InputAction::SelectedEntityCleared)),
      InputActionType::SetFilter => {
        let slot = Slot::read(r)?;
        let item = Item::from_u16(r.read_u16::<LittleEndian>()?).unwrap();
        Ok(Some(InputAction::SetFilter { slot, item, }))
      },
      InputActionType::SingleplayerInit => Ok(Some(InputAction::SingleplayerInit)),
      InputActionType::StackSplit => {  Ok(Some(InputAction::StackSplit { slot: Slot::read(r)? })) },
      InputActionType::StackTransfer => {  Ok(Some(InputAction::StackTransfer { slot: Slot::read(r)? })) },
      InputActionType::StartResearch => {  Ok(Some(InputAction::StartResearch { technology: r.read_u16::<LittleEndian>()?, })) },
      InputActionType::StartWalking => {  Ok(Some(InputAction::StartWalking { dir: Direction::from_u8(r.read_u8()?).unwrap() })) },
      InputActionType::StopBuildingByMoving => Ok(Some(InputAction::StopBuildingByMoving)),
      InputActionType::StopMining => Ok(Some(InputAction::StopMining)),
      InputActionType::StopWalking => Ok(Some(InputAction::StopWalking)),
      InputActionType::ToggleShowEntityInfo => Ok(Some(InputAction::ToggleShowEntityInfo)),
      InputActionType::UpdateBlueprintShelf => {
        r.read_u16_assert(0)?; // player id?
        r.read_u32_assert(1)?;
        r.read_u32::<LittleEndian>()?; // checksum
        let unknown_count = r.read_opt_u32()?;
        for _ in 0..unknown_count { r.read_u32::<LittleEndian>()?; } // dump unknown values
        let add_blueprint_record_data_count = r.read_opt_u32()?;
        for _ in 0..add_blueprint_record_data_count { r.read_past_add_blueprint_record_data()?; }
        let update_blueprint_data_count = r.read_opt_u32()?;
        for _ in 0..update_blueprint_data_count { r.read_past_update_blueprint_data()?; }
        println!("ignoring {} added and {} updated blueprints!", add_blueprint_record_data_count, update_blueprint_data_count);
        Ok(None)
      },
      _ => return Err(Error::new(ErrorKind::NotFound, format!("Unsupported action type {:?} at position {}", action_type, action_type_pos))),
    }
  }
  pub fn write(&self, w: &mut ReplayWriter) -> Result<()> {
    match self {
      InputAction::BeginMining => Ok(()),
      &InputAction::BuildItem { x, y, dir, ghost, } => {
        w.write_i32::<LittleEndian>(x)?;
        w.write_i32::<LittleEndian>(y)?;
        w.write_u8(dir.to_u8().unwrap())?;
        w.write_u8(1)?;
        w.write_u8(1)?;
        w.write_u8(if ghost { 1 } else { 0 })?;
        w.write_u8(0)
      },
      &InputAction::ChangeActiveItemGroupForCrafting { item_group, } => w.write_u8(item_group),
      InputAction::CleanCursorStack => Ok(()),
      InputAction::CloseGui => Ok(()),
      &InputAction::Craft { recipe, amount, } => {
        w.write_u16::<LittleEndian>(recipe.to_u16().unwrap())?;
        w.write_u32::<LittleEndian>(amount)
      },
      InputAction::CursorSplit { slot, } => slot.write(w),
      InputAction::CursorTransfer { slot, } => slot.write(w),
      InputAction::FastEntityTransfer { dir, } => w.write_u8(dir.to_u8().unwrap()),
      InputAction::FastEntitySplit { dir, } => w.write_u8(dir.to_u8().unwrap()),
      InputAction::GameCreatedFromScenario => Ok(()),
      InputAction::InventorySplit { slot, } => slot.write(w),
      InputAction::InventoryTransfer { slot, } => slot.write(w),
      InputAction::OpenCharacterGui => Ok(()),
      InputAction::OpenGui => Ok(()),
      InputAction::OpenTechnologyGui => Ok(()),
      InputAction::PlayerJoinGame { player_id, name, } => {
        w.write_opt_u16(*player_id)?;
        w.write_u16::<LittleEndian>(0)?;
        w.write_u8(1)?; // AllowedCommands
        w.write_string(name)?;
        w.write_u8(0)?;
        w.write_u8(1)
      },
      &InputAction::QuickBarPickSlot { slot, } => {
        w.write_u16::<LittleEndian>(slot)?;
        w.write_u8(0)?;
        w.write_u8(0)
      },
      InputAction::QuickBarSetSlot { slot,  source_slot, } => {
        w.write_u16::<LittleEndian>(*slot)?;
       source_slot.write(w)
      },
      &InputAction::SelectedEntityChanged { x, y, } => {
        w.write_i32::<LittleEndian>(x)?;
        w.write_i32::<LittleEndian>(y)
      },
      &InputAction::SelectedEntityChangedRelative { x, y, } => {
        w.write_i16::<LittleEndian>(y)?;
        w.write_i16::<LittleEndian>(x)
      },
      &InputAction::SelectedEntityChangedVeryClose { x, y, } => {
        assert!(x >= -8 && x < 8 && y >= -8 && y < 8);
        let xy = (((x + 8) as u8) << 4) | ((y + 8) as u8);
        w.write_u8(xy)
      },
      &InputAction::SelectedEntityChangedVeryClosePrecise { x, y, } => {
        w.write_i8(y)?;
        w.write_i8(x)
      },
      InputAction::SelectedEntityCleared => Ok(()),
      InputAction::SetFilter { slot, item, } => {
        slot.write(w)?;
        w.write_u16::<LittleEndian>(item.to_u16().unwrap())
      },
      InputAction::SingleplayerInit => Ok(()),
      InputAction::StackSplit { slot, } => slot.write(w),
      InputAction::StackTransfer { slot, } => slot.write(w),
      &InputAction::StartResearch { technology, } => w.write_u16::<LittleEndian>(technology),
      InputAction::StartWalking { dir, } => w.write_u8(dir.to_u8().unwrap()),
      InputAction::StopBuildingByMoving => Ok(()),
      InputAction::StopMining => Ok(()),
      InputAction::StopWalking => Ok(()),
      InputAction::ToggleShowEntityInfo => Ok(()),
    }
  }
  pub fn action_type(&self) -> InputActionType {
    match self {
      InputAction::BeginMining => InputActionType::BeginMining,
      InputAction::BuildItem { x: _, y: _, dir: _, ghost: _, } => InputActionType::BuildItem,
      InputAction::ChangeActiveItemGroupForCrafting { item_group: _, } => InputActionType::ChangeActiveItemGroupForCrafting,
      InputAction::CleanCursorStack => InputActionType::CleanCursorStack,
      InputAction::CloseGui => InputActionType::CloseGui,
      InputAction::Craft { recipe: _, amount: _, } => InputActionType::Craft,
      InputAction::CursorSplit { slot: _, } => InputActionType::CursorSplit,
      InputAction::CursorTransfer { slot: _, } => InputActionType::CursorTransfer,
      InputAction::FastEntitySplit { dir: _, } => InputActionType::FastEntitySplit,
      InputAction::FastEntityTransfer { dir: _, } => InputActionType::FastEntityTransfer,
      InputAction::GameCreatedFromScenario => InputActionType::GameCreatedFromScenario,
      InputAction::InventorySplit { slot: _, } => InputActionType::InventorySplit,
      InputAction::InventoryTransfer { slot: _, } => InputActionType::InventoryTransfer,
      InputAction::OpenCharacterGui => InputActionType::OpenCharacterGui,
      InputAction::OpenGui => InputActionType::OpenGui,
      InputAction::OpenTechnologyGui => InputActionType::OpenTechnologyGui,
      InputAction::PlayerJoinGame { player_id: _, name: _, } => InputActionType::PlayerJoinGame,
      InputAction::QuickBarPickSlot { slot: _, } => InputActionType::QuickBarPickSlot,
      InputAction::QuickBarSetSlot { slot: _,  source_slot: _, } => InputActionType::QuickBarSetSlot,
      InputAction::SelectedEntityChanged { x: _, y: _, } => InputActionType::SelectedEntityChanged,
      InputAction::SelectedEntityChangedRelative { x: _, y: _, } => InputActionType::SelectedEntityChangedRelative,
      InputAction::SelectedEntityChangedVeryClose { x: _, y: _, } => InputActionType::SelectedEntityChangedVeryClose,
      InputAction::SelectedEntityChangedVeryClosePrecise { x: _, y: _, } => InputActionType::SelectedEntityChangedVeryClosePrecise,
      InputAction::SelectedEntityCleared => InputActionType::SelectedEntityCleared,
      InputAction::SetFilter { slot: _, item: _, } => InputActionType::SetFilter,
      InputAction::SingleplayerInit => InputActionType::SingleplayerInit,
      InputAction::StackSplit { slot: _, } => InputActionType::StackSplit,
      InputAction::StackTransfer { slot: _, } => InputActionType::StackTransfer,
      InputAction::StartResearch { technology: _, } => InputActionType::StartResearch,
      InputAction::StartWalking { dir: _, } => InputActionType::StartWalking,
      InputAction::StopBuildingByMoving => InputActionType::StopBuildingByMoving,
      InputAction::StopMining => InputActionType::StopMining,
      InputAction::StopWalking => InputActionType::StopWalking,
      InputAction::ToggleShowEntityInfo => InputActionType::ToggleShowEntityInfo,
    }
  }
}
