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
