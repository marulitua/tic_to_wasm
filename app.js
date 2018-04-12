import { greet, render, Tile, Bar, MyResult, Foo } from "./tic_to_wasm";

// create grid and event handler based on collision detection
// inspired by http://jsfiddle.net/BmeKr/1800/

var elem = document.getElementById('myCanvas'),
  elemLeft = elem.offsetLeft,
  elemTop = elem.offsetTop,
  context = elem.getContext('2d'),
  elements = [],
  size = 20

// Set board size
elem.width = elem.height = size * 80 + size *  5 + 2 * 20

// Add event listener for `click` events.
elem.addEventListener('click', function (event) {
  var x = event.pageX - elemLeft,
    y = event.pageY - elemTop;

  elements.forEach(function (element) {
    if (y > element.top && y < element.top + element.height && x > element.left && x < element.left + element.width) {
    alert(element.text);
    }
  });
}, false);

// Set the value content (x,y) axis
var x = 20, y = 20, maxWidth = elem.getAttribute('width'),
  maxHeight = elem.getAttribute('height'),
  width = 80,
  height = 80,
  text = "",
  leftx = 0;

for (var i = 1; i <= size; i++) {
  y = 20;
  for (var j = 1; j <= size; j++) {
    if (i === j) {
      text = 'X'
    } else {
      text = 'O'
    }

    elements.push({
      colour: '#05EFFF',
      width: width,
      height: height,
      top: y,
      left: x,
      text: text,
      textColour: '#fff',
    });

    // get the y axis for next content
    y = y + elements[j-1].height + 5
    if (y >= maxHeight - elements[j-1].height) {
      break;
    }
  }
  //get the x axis for next content
  x = x + elements[0].width + 5
  if (x >= maxWidth - elements[0].width) {
    break;
  }
}

function render_tile(element) {
  context.font = "45pt Arial";
  context.strokeStyle = "#000";
  context.rect(element.left, element.top, element.width, element.height);
  context.fillText(element.text, element.left + element.width / 4, element.top+ element.height / 1.3);
  context.lineWidth = 1;
  context.stroke()
}

// Render elements.
//elements.forEach(render_tile);
greet("World!")
console.log(render())
let bar = Bar.from_str()
console.log(Bar, bar)
//let result = MyResult.new()
//console.log(result)
// Note the `new Foo()` syntax cannot be used, static function
// constructors must be used instead. Additionally objects allocated
// corresponding to Rust structs will need to be deallocated on the
// Rust side of things with an explicit call to `free`.
let foo = Foo.new();
console.log(foo)
foo.add(10)
console.log(foo)
foo.show()

foo.free();
console.log(foo)