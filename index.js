import {add} from './src/lib.rs';
console.log("hello world: " + add(2, 3));

// create grid and event handler based on collision detection
// inspired by http://jsfiddle.net/BmeKr/1800/

var elem = document.getElementById('myCanvas'),
  elemLeft = elem.offsetLeft,
  elemTop = elem.offsetTop,
  context = elem.getContext('2d'),
  elements = [];

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

var long = 3;
for (var i = 1; i <= long; i++) {
  y = 20;
  for (var j = 1; j <= long; j++) {
    text = i + ',' + j

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

// Render elements.
elements.forEach(function (element) {
  context.font = "14pt Arial";
  context.strokeStyle = "#000";
  context.rect(element.left, element.top, element.width, element.height);
  context.fillText(element.text, element.left + element.width / 2.5, element.top+ element.height / 1.7);
  context.lineWidth = 1;
  context.stroke()
});
console.log(elements)
