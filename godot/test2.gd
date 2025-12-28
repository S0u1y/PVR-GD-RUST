extends TextureButton

@onready var ContextMenu = $"../ItemList"

func _ready() -> void:
	if ContextMenu == null:
		printerr("Context menu has not been found in parent node.")

func _on_pressed() -> void:
	if ContextMenu == null: return
	
	ContextMenu.show_context(self.get_global_mouse_position(), self)
	
	if not mouse_exited.is_connected(_on_mouse_exited):
		mouse_exited.connect(_on_mouse_exited)
	

func _on_mouse_exited() -> void:
	if ContextMenu == null: return
	
	ContextMenu.hide_context()
	if mouse_exited.is_connected(_on_mouse_exited):
		mouse_exited.disconnect(_on_mouse_exited)
	

func _exit_tree() -> void:
	if ContextMenu != null and ContextMenu.get_parent() == self:
		ContextMenu.reparent(self.get_parent())
