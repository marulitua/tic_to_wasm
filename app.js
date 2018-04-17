import { next_move } from "./tic_to_wasm";

// DOM manipulation will be handle by js
// create grid and event handler based on collision detection
// inspired by http://jsfiddle.net/BmeKr/1800/

var elem = document.getElementById('myCanvas'),
  elemLeft = elem.offsetLeft,
  elemTop = elem.offsetTop,
  context = elem.getContext('2d'),
  elements = [],
  size = 3

var available_tiles = Math.pow(size, 2)

// Set board size
elem.width = elem.height = size * 80 + size *  5 + 2 * 20

// Add event listener for `click` events.
elem.addEventListener('click', function (event) {
  var x = event.pageX - elemLeft,
    y = event.pageY - elemTop;

  elements.forEach(function (element, index) {
    if (y > element.top && y < element.top + element.height && 
      x > element.left && x < element.left + element.width) {
      if (element.belongsTo === 'neutral') {
        element.belongsTo = 'human'
        element.text = '0'
        available_tiles -= 1;

        console.log(`available_tiles => ${available_tiles}`)
        if (available_tiles > 0) {
          let before = get_condition(elements);
          let robot_move = next_move(before)
          //console.log(`${string_c} => ${robot_move}`)
          let robot = elements[robot_move]
          if (robot.text === '0') {
            console.log('error detected')
          }
          robot.text = 'X'
          robot.belongsTo = 'robot'

          let after = get_condition(elements);
          elements.forEach(render_tile);
          console.log(`${before} => ${index} clicked ${robot_move} => ${after}`)
          available_tiles -= 1;
        }
      }
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
  x = 20;
  for (var j = 1; j <= size; j++) {

    elements.push({
      colour: '#05EFFF',
      width: width,
      height: height,
      top: y,
      left: x,
      text: '',//elements.length,
      textColour: '#fff',
      belongsTo: 'neutral'
    });

    //get the x axis for next content
    x = x + elements[0].width + 5;
    //console.log(`x => ${x} maxWidth => ${maxWidth}`);
  }
  // get the y axis for next content
  y = y + elements[0].height + 5
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
elements.forEach(render_tile);
//console.log(`next_move => ${next_move("1,0,0,0,0,0,0,0,0")}`);
//console.log(`next_move => ${next_move("1,2,0,1,1,0,0,0,0")}`);
function get_condition(elements) {
  var results = []

  elements.forEach(function(e,i) {
    if (e.belongsTo === 'human') {
      results.push(1)
    } else if (e.belongsTo === 'robot') {
      results.push(2)
    } else {
      results.push(0)
    }
  });

  return results.join(',')
}
