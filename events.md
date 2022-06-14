# Events

## KeyboardInput

### Writers



### Readers

* switch_edit_mode (..\brando\src\building\plugin.rs) -  Allow the user to switch edit mode base on the keyboard key
* move_camera_on_keyboard_input (..\brando\src\main.rs)

## PickingEvent

### Writers



### Readers

* build_building (..\brando\src\building\plugin.rs)

## MoreInhabitantsNeeded

### Writers

* try_spawn_inhabitants (..\brando\src\palatability\plugin.rs)

### Readers

* spawn_inhabitants (..\brando\src\navigation\plugin.rs)

## GameTick

### Writers

* tick (..\brando\src\main.rs)

### Readers

* make_progress (..\brando\src\building\plugin.rs)
* handle_waiting_for_inhabitants (..\brando\src\navigation\plugin.rs)
* move_inhabitants_to_house (..\brando\src\navigation\plugin.rs)
* try_spawn_inhabitants (..\brando\src\palatability\plugin.rs)

## InhabitantArrivedAtHomeEvent

### Writers

* move_inhabitants_to_house (..\brando\src\navigation\plugin.rs)

### Readers

* habit_house (..\brando\src\building\plugin.rs)
* habit_house (..\brando\src\palatability\plugin.rs)

## BuildingCreatedEvent

### Writers

* make_progress (..\brando\src\building\plugin.rs)

### Readers

* new_building_created (..\brando\src\navigation\plugin.rs)
* add_node (..\brando\src\navigation\plugin.rs)
* increment_palatabilities (..\brando\src\palatability\plugin.rs)
* listen_building_created (..\brando\src\palatability\plugin.rs)