#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting,
    MainMenu,
    SaveGame,
    Paused,
    ShowRemoveItem,
    GameOver,
    MapReveal,
    MapGeneration,
}
