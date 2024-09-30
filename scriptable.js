var json = new Request("http://sycamore:8888/widget").loadJSON();
var widget = new ListWidget();
var lines = await json;
lines.forEach(function (line) {
  widget.addText(line);
});
Script.setWidget(widget);
Script.complete();
widget.presentLarge();
