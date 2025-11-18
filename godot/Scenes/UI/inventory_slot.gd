extends TextureRect




func _on_gui_input(event: InputEvent) -> void:
	if (event as InputEventMouseButton):
		if event.button_mask == MouseButtonMask.MOUSE_BUTTON_MASK_RIGHT and event.is_pressed():
			var node = get_node("ItemOptions")
			node.visible = not node.visible
			node.global_position = get_global_mouse_position()
		
	
