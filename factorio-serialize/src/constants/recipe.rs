use enum_primitive_derive::Primitive;
use factorio_serialize_derive::ReplayReadWriteEnumU16;
use num_traits::{FromPrimitive, ToPrimitive};


// Version: 1.1.107
// Extraction method: util::export_prototypes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU16)]
pub enum Recipe {
  Accumulator = 1,
  AdvancedCircuit = 2,
  ArithmeticCombinator = 3,
  ArtilleryShell = 4,
  ArtilleryTargetingRemote = 5,
  ArtilleryTurret = 6,
  ArtilleryWagon = 7,
  AssemblingMachine1 = 8,
  AssemblingMachine2 = 9,
  AssemblingMachine3 = 10,
  AtomicBomb = 11,
  AutomationSciencePack = 12,
  Battery = 13,
  BatteryEquipment = 14,
  BatteryMk2Equipment = 15,
  Beacon = 16,
  BeltImmunityEquipment = 17,
  BigElectricPole = 18,
  Boiler = 19,
  BurnerInserter = 20,
  BurnerMiningDrill = 21,
  CannonShell = 22,
  Car = 23,
  CargoWagon = 24,
  Centrifuge = 25,
  ChemicalPlant = 26,
  ChemicalSciencePack = 27,
  CliffExplosives = 28,
  ClusterGrenade = 29,
  CombatShotgun = 30,
  Concrete = 31,
  ConstantCombinator = 32,
  ConstructionRobot = 33,
  CopperCable = 34,
  CopperPlate = 35,
  DeciderCombinator = 36,
  DefenderCapsule = 37,
  DestroyerCapsule = 38,
  DischargeDefenseEquipment = 39,
  DischargeDefenseRemote = 40,
  DistractorCapsule = 41,
  EffectivityModule = 42,
  EffectivityModule2 = 43,
  EffectivityModule3 = 44,
  ElectricEnergyInterface = 45,
  ElectricEngineUnit = 46,
  ElectricFurnace = 47,
  ElectricMiningDrill = 48,
  ElectronicCircuit = 49,
  EmptyBarrel = 50,
  EnergyShieldEquipment = 51,
  EnergyShieldMk2Equipment = 52,
  EngineUnit = 53,
  ExoskeletonEquipment = 54,
  ExplosiveCannonShell = 55,
  ExplosiveRocket = 56,
  ExplosiveUraniumCannonShell = 57,
  Explosives = 58,
  ExpressLoader = 59,
  ExpressSplitter = 60,
  ExpressTransportBelt = 61,
  ExpressUndergroundBelt = 62,
  FastInserter = 63,
  FastLoader = 64,
  FastSplitter = 65,
  FastTransportBelt = 66,
  FastUndergroundBelt = 67,
  FilterInserter = 68,
  FirearmMagazine = 69,
  Flamethrower = 70,
  FlamethrowerAmmo = 71,
  FlamethrowerTurret = 72,
  FluidWagon = 73,
  FlyingRobotFrame = 74,
  FusionReactorEquipment = 75,
  Gate = 76,
  GreenWire = 77,
  Grenade = 78,
  GunTurret = 79,
  HazardConcrete = 80,
  HeatExchanger = 81,
  HeatPipe = 82,
  HeavyArmor = 83,
  Inserter = 84,
  IronChest = 85,
  IronGearWheel = 86,
  IronPlate = 87,
  IronStick = 88,
  Lab = 89,
  LandMine = 90,
  Landfill = 91,
  LaserTurret = 92,
  LightArmor = 93,
  Loader = 94,
  Locomotive = 95,
  LogisticChestActiveProvider = 96,
  LogisticChestBuffer = 97,
  LogisticChestPassiveProvider = 98,
  LogisticChestRequester = 99,
  LogisticChestStorage = 100,
  LogisticRobot = 101,
  LogisticSciencePack = 102,
  LongHandedInserter = 103,
  LowDensityStructure = 104,
  Lubricant = 105,
  MediumElectricPole = 106,
  MilitarySciencePack = 107,
  ModularArmor = 108,
  NightVisionEquipment = 109,
  NuclearFuel = 110,
  NuclearReactor = 111,
  OffshorePump = 112,
  OilRefinery = 113,
  PersonalLaserDefenseEquipment = 114,
  PersonalRoboportEquipment = 115,
  PersonalRoboportMk2Equipment = 116,
  PiercingRoundsMagazine = 117,
  PiercingShotgunShell = 118,
  Pipe = 119,
  PipeToGround = 120,
  Pistol = 121,
  PlasticBar = 122,
  PoisonCapsule = 123,
  PowerArmor = 124,
  PowerArmorMk2 = 125,
  PowerSwitch = 126,
  ProcessingUnit = 127,
  ProductionSciencePack = 128,
  ProductivityModule = 129,
  ProductivityModule2 = 130,
  ProductivityModule3 = 131,
  ProgrammableSpeaker = 132,
  Pump = 133,
  Pumpjack = 134,
  Radar = 135,
  Rail = 136,
  RailChainSignal = 137,
  RailSignal = 138,
  RedWire = 139,
  RefinedConcrete = 140,
  RefinedHazardConcrete = 141,
  RepairPack = 142,
  Roboport = 143,
  Rocket = 144,
  RocketControlUnit = 145,
  RocketFuel = 146,
  RocketLauncher = 147,
  RocketPart = 148,
  RocketSilo = 149,
  Satellite = 150,
  Shotgun = 151,
  ShotgunShell = 152,
  SlowdownCapsule = 153,
  SmallElectricPole = 154,
  SmallLamp = 155,
  SolarPanel = 156,
  SolarPanelEquipment = 157,
  SpeedModule = 158,
  SpeedModule2 = 159,
  SpeedModule3 = 160,
  Spidertron = 161,
  SpidertronRemote = 162,
  Splitter = 163,
  StackFilterInserter = 164,
  StackInserter = 165,
  SteamEngine = 166,
  SteamTurbine = 167,
  SteelChest = 168,
  SteelFurnace = 169,
  SteelPlate = 170,
  StoneBrick = 171,
  StoneFurnace = 172,
  StoneWall = 173,
  StorageTank = 174,
  SubmachineGun = 175,
  Substation = 176,
  Sulfur = 177,
  SulfuricAcid = 178,
  Tank = 179,
  TrainStop = 180,
  TransportBelt = 181,
  UndergroundBelt = 182,
  UraniumCannonShell = 183,
  UraniumFuelCell = 184,
  UraniumRoundsMagazine = 185,
  UtilitySciencePack = 186,
  WoodenChest = 187,
  BasicOilProcessing = 188,
  AdvancedOilProcessing = 189,
  CoalLiquefaction = 190,
  FillCrudeOilBarrel = 191,
  FillHeavyOilBarrel = 192,
  FillLightOilBarrel = 193,
  FillLubricantBarrel = 194,
  FillPetroleumGasBarrel = 195,
  FillSulfuricAcidBarrel = 196,
  FillWaterBarrel = 197,
  HeavyOilCracking = 198,
  LightOilCracking = 199,
  SolidFuelFromLightOil = 200,
  SolidFuelFromPetroleumGas = 201,
  SolidFuelFromHeavyOil = 202,
  EmptyCrudeOilBarrel = 203,
  EmptyHeavyOilBarrel = 204,
  EmptyLightOilBarrel = 205,
  EmptyLubricantBarrel = 206,
  EmptyPetroleumGasBarrel = 207,
  EmptySulfuricAcidBarrel = 208,
  EmptyWaterBarrel = 209,
  UraniumProcessing = 210,
  NuclearFuelReprocessing = 211,
  KovarexEnrichmentProcess = 212,
}
impl Recipe {
  pub fn name(self) -> &'static str {
    match self {
      Recipe::Accumulator => "accumulator",
      Recipe::AdvancedCircuit => "advanced-circuit",
      Recipe::ArithmeticCombinator => "arithmetic-combinator",
      Recipe::ArtilleryShell => "artillery-shell",
      Recipe::ArtilleryTargetingRemote => "artillery-targeting-remote",
      Recipe::ArtilleryTurret => "artillery-turret",
      Recipe::ArtilleryWagon => "artillery-wagon",
      Recipe::AssemblingMachine1 => "assembling-machine-1",
      Recipe::AssemblingMachine2 => "assembling-machine-2",
      Recipe::AssemblingMachine3 => "assembling-machine-3",
      Recipe::AtomicBomb => "atomic-bomb",
      Recipe::AutomationSciencePack => "automation-science-pack",
      Recipe::Battery => "battery",
      Recipe::BatteryEquipment => "battery-equipment",
      Recipe::BatteryMk2Equipment => "battery-mk2-equipment",
      Recipe::Beacon => "beacon",
      Recipe::BeltImmunityEquipment => "belt-immunity-equipment",
      Recipe::BigElectricPole => "big-electric-pole",
      Recipe::Boiler => "boiler",
      Recipe::BurnerInserter => "burner-inserter",
      Recipe::BurnerMiningDrill => "burner-mining-drill",
      Recipe::CannonShell => "cannon-shell",
      Recipe::Car => "car",
      Recipe::CargoWagon => "cargo-wagon",
      Recipe::Centrifuge => "centrifuge",
      Recipe::ChemicalPlant => "chemical-plant",
      Recipe::ChemicalSciencePack => "chemical-science-pack",
      Recipe::CliffExplosives => "cliff-explosives",
      Recipe::ClusterGrenade => "cluster-grenade",
      Recipe::CombatShotgun => "combat-shotgun",
      Recipe::Concrete => "concrete",
      Recipe::ConstantCombinator => "constant-combinator",
      Recipe::ConstructionRobot => "construction-robot",
      Recipe::CopperCable => "copper-cable",
      Recipe::CopperPlate => "copper-plate",
      Recipe::DeciderCombinator => "decider-combinator",
      Recipe::DefenderCapsule => "defender-capsule",
      Recipe::DestroyerCapsule => "destroyer-capsule",
      Recipe::DischargeDefenseEquipment => "discharge-defense-equipment",
      Recipe::DischargeDefenseRemote => "discharge-defense-remote",
      Recipe::DistractorCapsule => "distractor-capsule",
      Recipe::EffectivityModule => "effectivity-module",
      Recipe::EffectivityModule2 => "effectivity-module-2",
      Recipe::EffectivityModule3 => "effectivity-module-3",
      Recipe::ElectricEnergyInterface => "electric-energy-interface",
      Recipe::ElectricEngineUnit => "electric-engine-unit",
      Recipe::ElectricFurnace => "electric-furnace",
      Recipe::ElectricMiningDrill => "electric-mining-drill",
      Recipe::ElectronicCircuit => "electronic-circuit",
      Recipe::EmptyBarrel => "empty-barrel",
      Recipe::EnergyShieldEquipment => "energy-shield-equipment",
      Recipe::EnergyShieldMk2Equipment => "energy-shield-mk2-equipment",
      Recipe::EngineUnit => "engine-unit",
      Recipe::ExoskeletonEquipment => "exoskeleton-equipment",
      Recipe::ExplosiveCannonShell => "explosive-cannon-shell",
      Recipe::ExplosiveRocket => "explosive-rocket",
      Recipe::ExplosiveUraniumCannonShell => "explosive-uranium-cannon-shell",
      Recipe::Explosives => "explosives",
      Recipe::ExpressLoader => "express-loader",
      Recipe::ExpressSplitter => "express-splitter",
      Recipe::ExpressTransportBelt => "express-transport-belt",
      Recipe::ExpressUndergroundBelt => "express-underground-belt",
      Recipe::FastInserter => "fast-inserter",
      Recipe::FastLoader => "fast-loader",
      Recipe::FastSplitter => "fast-splitter",
      Recipe::FastTransportBelt => "fast-transport-belt",
      Recipe::FastUndergroundBelt => "fast-underground-belt",
      Recipe::FilterInserter => "filter-inserter",
      Recipe::FirearmMagazine => "firearm-magazine",
      Recipe::Flamethrower => "flamethrower",
      Recipe::FlamethrowerAmmo => "flamethrower-ammo",
      Recipe::FlamethrowerTurret => "flamethrower-turret",
      Recipe::FluidWagon => "fluid-wagon",
      Recipe::FlyingRobotFrame => "flying-robot-frame",
      Recipe::FusionReactorEquipment => "fusion-reactor-equipment",
      Recipe::Gate => "gate",
      Recipe::GreenWire => "green-wire",
      Recipe::Grenade => "grenade",
      Recipe::GunTurret => "gun-turret",
      Recipe::HazardConcrete => "hazard-concrete",
      Recipe::HeatExchanger => "heat-exchanger",
      Recipe::HeatPipe => "heat-pipe",
      Recipe::HeavyArmor => "heavy-armor",
      Recipe::Inserter => "inserter",
      Recipe::IronChest => "iron-chest",
      Recipe::IronGearWheel => "iron-gear-wheel",
      Recipe::IronPlate => "iron-plate",
      Recipe::IronStick => "iron-stick",
      Recipe::Lab => "lab",
      Recipe::LandMine => "land-mine",
      Recipe::Landfill => "landfill",
      Recipe::LaserTurret => "laser-turret",
      Recipe::LightArmor => "light-armor",
      Recipe::Loader => "loader",
      Recipe::Locomotive => "locomotive",
      Recipe::LogisticChestActiveProvider => "logistic-chest-active-provider",
      Recipe::LogisticChestBuffer => "logistic-chest-buffer",
      Recipe::LogisticChestPassiveProvider => "logistic-chest-passive-provider",
      Recipe::LogisticChestRequester => "logistic-chest-requester",
      Recipe::LogisticChestStorage => "logistic-chest-storage",
      Recipe::LogisticRobot => "logistic-robot",
      Recipe::LogisticSciencePack => "logistic-science-pack",
      Recipe::LongHandedInserter => "long-handed-inserter",
      Recipe::LowDensityStructure => "low-density-structure",
      Recipe::Lubricant => "lubricant",
      Recipe::MediumElectricPole => "medium-electric-pole",
      Recipe::MilitarySciencePack => "military-science-pack",
      Recipe::ModularArmor => "modular-armor",
      Recipe::NightVisionEquipment => "night-vision-equipment",
      Recipe::NuclearFuel => "nuclear-fuel",
      Recipe::NuclearReactor => "nuclear-reactor",
      Recipe::OffshorePump => "offshore-pump",
      Recipe::OilRefinery => "oil-refinery",
      Recipe::PersonalLaserDefenseEquipment => "personal-laser-defense-equipment",
      Recipe::PersonalRoboportEquipment => "personal-roboport-equipment",
      Recipe::PersonalRoboportMk2Equipment => "personal-roboport-mk2-equipment",
      Recipe::PiercingRoundsMagazine => "piercing-rounds-magazine",
      Recipe::PiercingShotgunShell => "piercing-shotgun-shell",
      Recipe::Pipe => "pipe",
      Recipe::PipeToGround => "pipe-to-ground",
      Recipe::Pistol => "pistol",
      Recipe::PlasticBar => "plastic-bar",
      Recipe::PoisonCapsule => "poison-capsule",
      Recipe::PowerArmor => "power-armor",
      Recipe::PowerArmorMk2 => "power-armor-mk2",
      Recipe::PowerSwitch => "power-switch",
      Recipe::ProcessingUnit => "processing-unit",
      Recipe::ProductionSciencePack => "production-science-pack",
      Recipe::ProductivityModule => "productivity-module",
      Recipe::ProductivityModule2 => "productivity-module-2",
      Recipe::ProductivityModule3 => "productivity-module-3",
      Recipe::ProgrammableSpeaker => "programmable-speaker",
      Recipe::Pump => "pump",
      Recipe::Pumpjack => "pumpjack",
      Recipe::Radar => "radar",
      Recipe::Rail => "rail",
      Recipe::RailChainSignal => "rail-chain-signal",
      Recipe::RailSignal => "rail-signal",
      Recipe::RedWire => "red-wire",
      Recipe::RefinedConcrete => "refined-concrete",
      Recipe::RefinedHazardConcrete => "refined-hazard-concrete",
      Recipe::RepairPack => "repair-pack",
      Recipe::Roboport => "roboport",
      Recipe::Rocket => "rocket",
      Recipe::RocketControlUnit => "rocket-control-unit",
      Recipe::RocketFuel => "rocket-fuel",
      Recipe::RocketLauncher => "rocket-launcher",
      Recipe::RocketPart => "rocket-part",
      Recipe::RocketSilo => "rocket-silo",
      Recipe::Satellite => "satellite",
      Recipe::Shotgun => "shotgun",
      Recipe::ShotgunShell => "shotgun-shell",
      Recipe::SlowdownCapsule => "slowdown-capsule",
      Recipe::SmallElectricPole => "small-electric-pole",
      Recipe::SmallLamp => "small-lamp",
      Recipe::SolarPanel => "solar-panel",
      Recipe::SolarPanelEquipment => "solar-panel-equipment",
      Recipe::SpeedModule => "speed-module",
      Recipe::SpeedModule2 => "speed-module-2",
      Recipe::SpeedModule3 => "speed-module-3",
      Recipe::Spidertron => "spidertron",
      Recipe::SpidertronRemote => "spidertron-remote",
      Recipe::Splitter => "splitter",
      Recipe::StackFilterInserter => "stack-filter-inserter",
      Recipe::StackInserter => "stack-inserter",
      Recipe::SteamEngine => "steam-engine",
      Recipe::SteamTurbine => "steam-turbine",
      Recipe::SteelChest => "steel-chest",
      Recipe::SteelFurnace => "steel-furnace",
      Recipe::SteelPlate => "steel-plate",
      Recipe::StoneBrick => "stone-brick",
      Recipe::StoneFurnace => "stone-furnace",
      Recipe::StoneWall => "stone-wall",
      Recipe::StorageTank => "storage-tank",
      Recipe::SubmachineGun => "submachine-gun",
      Recipe::Substation => "substation",
      Recipe::Sulfur => "sulfur",
      Recipe::SulfuricAcid => "sulfuric-acid",
      Recipe::Tank => "tank",
      Recipe::TrainStop => "train-stop",
      Recipe::TransportBelt => "transport-belt",
      Recipe::UndergroundBelt => "underground-belt",
      Recipe::UraniumCannonShell => "uranium-cannon-shell",
      Recipe::UraniumFuelCell => "uranium-fuel-cell",
      Recipe::UraniumRoundsMagazine => "uranium-rounds-magazine",
      Recipe::UtilitySciencePack => "utility-science-pack",
      Recipe::WoodenChest => "wooden-chest",
      Recipe::BasicOilProcessing => "basic-oil-processing",
      Recipe::AdvancedOilProcessing => "advanced-oil-processing",
      Recipe::CoalLiquefaction => "coal-liquefaction",
      Recipe::FillCrudeOilBarrel => "fill-crude-oil-barrel",
      Recipe::FillHeavyOilBarrel => "fill-heavy-oil-barrel",
      Recipe::FillLightOilBarrel => "fill-light-oil-barrel",
      Recipe::FillLubricantBarrel => "fill-lubricant-barrel",
      Recipe::FillPetroleumGasBarrel => "fill-petroleum-gas-barrel",
      Recipe::FillSulfuricAcidBarrel => "fill-sulfuric-acid-barrel",
      Recipe::FillWaterBarrel => "fill-water-barrel",
      Recipe::HeavyOilCracking => "heavy-oil-cracking",
      Recipe::LightOilCracking => "light-oil-cracking",
      Recipe::SolidFuelFromLightOil => "solid-fuel-from-light-oil",
      Recipe::SolidFuelFromPetroleumGas => "solid-fuel-from-petroleum-gas",
      Recipe::SolidFuelFromHeavyOil => "solid-fuel-from-heavy-oil",
      Recipe::EmptyCrudeOilBarrel => "empty-crude-oil-barrel",
      Recipe::EmptyHeavyOilBarrel => "empty-heavy-oil-barrel",
      Recipe::EmptyLightOilBarrel => "empty-light-oil-barrel",
      Recipe::EmptyLubricantBarrel => "empty-lubricant-barrel",
      Recipe::EmptyPetroleumGasBarrel => "empty-petroleum-gas-barrel",
      Recipe::EmptySulfuricAcidBarrel => "empty-sulfuric-acid-barrel",
      Recipe::EmptyWaterBarrel => "empty-water-barrel",
      Recipe::UraniumProcessing => "uranium-processing",
      Recipe::NuclearFuelReprocessing => "nuclear-fuel-reprocessing",
      Recipe::KovarexEnrichmentProcess => "kovarex-enrichment-process",
    }
  }
  pub fn from_name(name: &str) -> Recipe {
    match name {
      "accumulator" => Recipe::Accumulator,
      "advanced-circuit" => Recipe::AdvancedCircuit,
      "arithmetic-combinator" => Recipe::ArithmeticCombinator,
      "artillery-shell" => Recipe::ArtilleryShell,
      "artillery-targeting-remote" => Recipe::ArtilleryTargetingRemote,
      "artillery-turret" => Recipe::ArtilleryTurret,
      "artillery-wagon" => Recipe::ArtilleryWagon,
      "assembling-machine-1" => Recipe::AssemblingMachine1,
      "assembling-machine-2" => Recipe::AssemblingMachine2,
      "assembling-machine-3" => Recipe::AssemblingMachine3,
      "atomic-bomb" => Recipe::AtomicBomb,
      "automation-science-pack" => Recipe::AutomationSciencePack,
      "battery" => Recipe::Battery,
      "battery-equipment" => Recipe::BatteryEquipment,
      "battery-mk2-equipment" => Recipe::BatteryMk2Equipment,
      "beacon" => Recipe::Beacon,
      "belt-immunity-equipment" => Recipe::BeltImmunityEquipment,
      "big-electric-pole" => Recipe::BigElectricPole,
      "boiler" => Recipe::Boiler,
      "burner-inserter" => Recipe::BurnerInserter,
      "burner-mining-drill" => Recipe::BurnerMiningDrill,
      "cannon-shell" => Recipe::CannonShell,
      "car" => Recipe::Car,
      "cargo-wagon" => Recipe::CargoWagon,
      "centrifuge" => Recipe::Centrifuge,
      "chemical-plant" => Recipe::ChemicalPlant,
      "chemical-science-pack" => Recipe::ChemicalSciencePack,
      "cliff-explosives" => Recipe::CliffExplosives,
      "cluster-grenade" => Recipe::ClusterGrenade,
      "combat-shotgun" => Recipe::CombatShotgun,
      "concrete" => Recipe::Concrete,
      "constant-combinator" => Recipe::ConstantCombinator,
      "construction-robot" => Recipe::ConstructionRobot,
      "copper-cable" => Recipe::CopperCable,
      "copper-plate" => Recipe::CopperPlate,
      "decider-combinator" => Recipe::DeciderCombinator,
      "defender-capsule" => Recipe::DefenderCapsule,
      "destroyer-capsule" => Recipe::DestroyerCapsule,
      "discharge-defense-equipment" => Recipe::DischargeDefenseEquipment,
      "discharge-defense-remote" => Recipe::DischargeDefenseRemote,
      "distractor-capsule" => Recipe::DistractorCapsule,
      "effectivity-module" => Recipe::EffectivityModule,
      "effectivity-module-2" => Recipe::EffectivityModule2,
      "effectivity-module-3" => Recipe::EffectivityModule3,
      "electric-energy-interface" => Recipe::ElectricEnergyInterface,
      "electric-engine-unit" => Recipe::ElectricEngineUnit,
      "electric-furnace" => Recipe::ElectricFurnace,
      "electric-mining-drill" => Recipe::ElectricMiningDrill,
      "electronic-circuit" => Recipe::ElectronicCircuit,
      "empty-barrel" => Recipe::EmptyBarrel,
      "energy-shield-equipment" => Recipe::EnergyShieldEquipment,
      "energy-shield-mk2-equipment" => Recipe::EnergyShieldMk2Equipment,
      "engine-unit" => Recipe::EngineUnit,
      "exoskeleton-equipment" => Recipe::ExoskeletonEquipment,
      "explosive-cannon-shell" => Recipe::ExplosiveCannonShell,
      "explosive-rocket" => Recipe::ExplosiveRocket,
      "explosive-uranium-cannon-shell" => Recipe::ExplosiveUraniumCannonShell,
      "explosives" => Recipe::Explosives,
      "express-loader" => Recipe::ExpressLoader,
      "express-splitter" => Recipe::ExpressSplitter,
      "express-transport-belt" => Recipe::ExpressTransportBelt,
      "express-underground-belt" => Recipe::ExpressUndergroundBelt,
      "fast-inserter" => Recipe::FastInserter,
      "fast-loader" => Recipe::FastLoader,
      "fast-splitter" => Recipe::FastSplitter,
      "fast-transport-belt" => Recipe::FastTransportBelt,
      "fast-underground-belt" => Recipe::FastUndergroundBelt,
      "filter-inserter" => Recipe::FilterInserter,
      "firearm-magazine" => Recipe::FirearmMagazine,
      "flamethrower" => Recipe::Flamethrower,
      "flamethrower-ammo" => Recipe::FlamethrowerAmmo,
      "flamethrower-turret" => Recipe::FlamethrowerTurret,
      "fluid-wagon" => Recipe::FluidWagon,
      "flying-robot-frame" => Recipe::FlyingRobotFrame,
      "fusion-reactor-equipment" => Recipe::FusionReactorEquipment,
      "gate" => Recipe::Gate,
      "green-wire" => Recipe::GreenWire,
      "grenade" => Recipe::Grenade,
      "gun-turret" => Recipe::GunTurret,
      "hazard-concrete" => Recipe::HazardConcrete,
      "heat-exchanger" => Recipe::HeatExchanger,
      "heat-pipe" => Recipe::HeatPipe,
      "heavy-armor" => Recipe::HeavyArmor,
      "inserter" => Recipe::Inserter,
      "iron-chest" => Recipe::IronChest,
      "iron-gear-wheel" => Recipe::IronGearWheel,
      "iron-plate" => Recipe::IronPlate,
      "iron-stick" => Recipe::IronStick,
      "lab" => Recipe::Lab,
      "land-mine" => Recipe::LandMine,
      "landfill" => Recipe::Landfill,
      "laser-turret" => Recipe::LaserTurret,
      "light-armor" => Recipe::LightArmor,
      "loader" => Recipe::Loader,
      "locomotive" => Recipe::Locomotive,
      "logistic-chest-active-provider" => Recipe::LogisticChestActiveProvider,
      "logistic-chest-buffer" => Recipe::LogisticChestBuffer,
      "logistic-chest-passive-provider" => Recipe::LogisticChestPassiveProvider,
      "logistic-chest-requester" => Recipe::LogisticChestRequester,
      "logistic-chest-storage" => Recipe::LogisticChestStorage,
      "logistic-robot" => Recipe::LogisticRobot,
      "logistic-science-pack" => Recipe::LogisticSciencePack,
      "long-handed-inserter" => Recipe::LongHandedInserter,
      "low-density-structure" => Recipe::LowDensityStructure,
      "lubricant" => Recipe::Lubricant,
      "medium-electric-pole" => Recipe::MediumElectricPole,
      "military-science-pack" => Recipe::MilitarySciencePack,
      "modular-armor" => Recipe::ModularArmor,
      "night-vision-equipment" => Recipe::NightVisionEquipment,
      "nuclear-fuel" => Recipe::NuclearFuel,
      "nuclear-reactor" => Recipe::NuclearReactor,
      "offshore-pump" => Recipe::OffshorePump,
      "oil-refinery" => Recipe::OilRefinery,
      "personal-laser-defense-equipment" => Recipe::PersonalLaserDefenseEquipment,
      "personal-roboport-equipment" => Recipe::PersonalRoboportEquipment,
      "personal-roboport-mk2-equipment" => Recipe::PersonalRoboportMk2Equipment,
      "piercing-rounds-magazine" => Recipe::PiercingRoundsMagazine,
      "piercing-shotgun-shell" => Recipe::PiercingShotgunShell,
      "pipe" => Recipe::Pipe,
      "pipe-to-ground" => Recipe::PipeToGround,
      "pistol" => Recipe::Pistol,
      "plastic-bar" => Recipe::PlasticBar,
      "poison-capsule" => Recipe::PoisonCapsule,
      "power-armor" => Recipe::PowerArmor,
      "power-armor-mk2" => Recipe::PowerArmorMk2,
      "power-switch" => Recipe::PowerSwitch,
      "processing-unit" => Recipe::ProcessingUnit,
      "production-science-pack" => Recipe::ProductionSciencePack,
      "productivity-module" => Recipe::ProductivityModule,
      "productivity-module-2" => Recipe::ProductivityModule2,
      "productivity-module-3" => Recipe::ProductivityModule3,
      "programmable-speaker" => Recipe::ProgrammableSpeaker,
      "pump" => Recipe::Pump,
      "pumpjack" => Recipe::Pumpjack,
      "radar" => Recipe::Radar,
      "rail" => Recipe::Rail,
      "rail-chain-signal" => Recipe::RailChainSignal,
      "rail-signal" => Recipe::RailSignal,
      "red-wire" => Recipe::RedWire,
      "refined-concrete" => Recipe::RefinedConcrete,
      "refined-hazard-concrete" => Recipe::RefinedHazardConcrete,
      "repair-pack" => Recipe::RepairPack,
      "roboport" => Recipe::Roboport,
      "rocket" => Recipe::Rocket,
      "rocket-control-unit" => Recipe::RocketControlUnit,
      "rocket-fuel" => Recipe::RocketFuel,
      "rocket-launcher" => Recipe::RocketLauncher,
      "rocket-part" => Recipe::RocketPart,
      "rocket-silo" => Recipe::RocketSilo,
      "satellite" => Recipe::Satellite,
      "shotgun" => Recipe::Shotgun,
      "shotgun-shell" => Recipe::ShotgunShell,
      "slowdown-capsule" => Recipe::SlowdownCapsule,
      "small-electric-pole" => Recipe::SmallElectricPole,
      "small-lamp" => Recipe::SmallLamp,
      "solar-panel" => Recipe::SolarPanel,
      "solar-panel-equipment" => Recipe::SolarPanelEquipment,
      "speed-module" => Recipe::SpeedModule,
      "speed-module-2" => Recipe::SpeedModule2,
      "speed-module-3" => Recipe::SpeedModule3,
      "spidertron" => Recipe::Spidertron,
      "spidertron-remote" => Recipe::SpidertronRemote,
      "splitter" => Recipe::Splitter,
      "stack-filter-inserter" => Recipe::StackFilterInserter,
      "stack-inserter" => Recipe::StackInserter,
      "steam-engine" => Recipe::SteamEngine,
      "steam-turbine" => Recipe::SteamTurbine,
      "steel-chest" => Recipe::SteelChest,
      "steel-furnace" => Recipe::SteelFurnace,
      "steel-plate" => Recipe::SteelPlate,
      "stone-brick" => Recipe::StoneBrick,
      "stone-furnace" => Recipe::StoneFurnace,
      "stone-wall" => Recipe::StoneWall,
      "storage-tank" => Recipe::StorageTank,
      "submachine-gun" => Recipe::SubmachineGun,
      "substation" => Recipe::Substation,
      "sulfur" => Recipe::Sulfur,
      "sulfuric-acid" => Recipe::SulfuricAcid,
      "tank" => Recipe::Tank,
      "train-stop" => Recipe::TrainStop,
      "transport-belt" => Recipe::TransportBelt,
      "underground-belt" => Recipe::UndergroundBelt,
      "uranium-cannon-shell" => Recipe::UraniumCannonShell,
      "uranium-fuel-cell" => Recipe::UraniumFuelCell,
      "uranium-rounds-magazine" => Recipe::UraniumRoundsMagazine,
      "utility-science-pack" => Recipe::UtilitySciencePack,
      "wooden-chest" => Recipe::WoodenChest,
      "basic-oil-processing" => Recipe::BasicOilProcessing,
      "advanced-oil-processing" => Recipe::AdvancedOilProcessing,
      "coal-liquefaction" => Recipe::CoalLiquefaction,
      "fill-crude-oil-barrel" => Recipe::FillCrudeOilBarrel,
      "fill-heavy-oil-barrel" => Recipe::FillHeavyOilBarrel,
      "fill-light-oil-barrel" => Recipe::FillLightOilBarrel,
      "fill-lubricant-barrel" => Recipe::FillLubricantBarrel,
      "fill-petroleum-gas-barrel" => Recipe::FillPetroleumGasBarrel,
      "fill-sulfuric-acid-barrel" => Recipe::FillSulfuricAcidBarrel,
      "fill-water-barrel" => Recipe::FillWaterBarrel,
      "heavy-oil-cracking" => Recipe::HeavyOilCracking,
      "light-oil-cracking" => Recipe::LightOilCracking,
      "solid-fuel-from-light-oil" => Recipe::SolidFuelFromLightOil,
      "solid-fuel-from-petroleum-gas" => Recipe::SolidFuelFromPetroleumGas,
      "solid-fuel-from-heavy-oil" => Recipe::SolidFuelFromHeavyOil,
      "empty-crude-oil-barrel" => Recipe::EmptyCrudeOilBarrel,
      "empty-heavy-oil-barrel" => Recipe::EmptyHeavyOilBarrel,
      "empty-light-oil-barrel" => Recipe::EmptyLightOilBarrel,
      "empty-lubricant-barrel" => Recipe::EmptyLubricantBarrel,
      "empty-petroleum-gas-barrel" => Recipe::EmptyPetroleumGasBarrel,
      "empty-sulfuric-acid-barrel" => Recipe::EmptySulfuricAcidBarrel,
      "empty-water-barrel" => Recipe::EmptyWaterBarrel,
      "uranium-processing" => Recipe::UraniumProcessing,
      "nuclear-fuel-reprocessing" => Recipe::NuclearFuelReprocessing,
      "kovarex-enrichment-process" => Recipe::KovarexEnrichmentProcess,
      name => panic!("unknown Recipe \"{name}\""),
    }
  }
}
