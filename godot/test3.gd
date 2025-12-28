extends ItemList

var context_options: Node

func show_context(_position: Vector2, node: Node):
	self.hide_context()
	
	context_options = node.get_node("ContextOptions")
	if not context_options: return
	
	for child in context_options.get_children():
		self.add_item(child.name)
	
	self.reparent(node)
	self.set_global_position(_position)
	
	self.visible = true

func _on_item_selected(index: int) -> void:
	var selected_option = context_options.get_child(index)
	print("Selected ", selected_option.name)
	selected_option.select()
	

func hide_context():
	self.visible = false
	context_options = null
	self.clear()
