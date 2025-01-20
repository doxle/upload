const canvas = document.getElementById("drawingCanvas");
const ctx = canvas.getContext("2d");

let isDrawing = false;
let startX = 0;
let startY = 0;

function getTouchPosition(canvas, touchEvent) {
    const rect = canvas.getBoundingClientRect();
    const touch = touchEvent.touches[0] || touchEvent.changedTouches[0];
    return {
      x: touch.clientX - rect.left,
      y: touch.clientY - rect.top
    };
}

//WEB
canvas.addEventListener('mousedown', (event) => {
  isDrawing = true;
  startX = event.offsetX;
  startY = event.offsetY;
});

//MOBILE
canvas.addEventListener('touchstart', (event) => {
  isDrawing = true;
  const touchPos = getTouchPosition(canvas, event)
  startX = touchPos.offsetX;
  startY = touchPos.offsetY;
  // TO PREVENT ANY SCROLLING
  event.preventDefault();
});

canvas.addEventListener('mousemove', event => {
  if (isDrawing) {
    const endX = event.offsetX;
    const endY = event.offsetY;
      ctx.beginPath();
      ctx.moveTo(startX, startY);
      ctx.lineTo(endX, endY);
      ctx.stroke();
      startX = endX;
      startY = endY;
  }
});

canvas.addEventListener('touchmove', event => {
  if (isDrawing) {
    const touchPos = getTouchPosition(canvas, event);
    const endX = touchPos.x;
    const endY = touchPos.y;
    ctx.beginPath();
    ctx.moveTo(startX, startY);
    ctx.lineTo(endX, endY);
    ctx.stroke();
    startX = endX;
    startY = endY;

    event.preventDefault();
  }
});

canvas.addEventListener('mouseup', () => {
  isDrawing = false;
});

canvas.addEventListener('mouseleave', () => {
  isDrawing = false;
});

canvas.addEventListener('touchend', () => {
  isDrawing = false;
});

canvas.addEventListener('touchcancel', () => {
  isDrawing = false;
});


const Konva = require('konva');

// Example usage
const stage = new Konva.Stage({
  container: 'container',
  width: 500,
  height: 500,
});

const layer = new Konva.Layer();
stage.add(layer);

const circle = new Konva.Circle({
  x: stage.width() / 2,
  y: stage.height() / 2,
  radius: 70,
  fill: 'red',
  stroke: 'black',
  strokeWidth: 4,
});

layer.add(circle);
layer.draw();


// <script src="https://cdn.jsdelivr.net/npm/konva@8.4.3/konva.min.js"></script>
// <script src="your-script.js"></script>
// </body>
// </html
