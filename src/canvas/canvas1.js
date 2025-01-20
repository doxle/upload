const canvas = getElementById("drawingCanvas");
const ctx = canvas.getContext("2d");

let startX = 0;
let startY = 0;

canvas.addEventListener("mousedown",(event)=>{
    // REGISTERING THE STARTING POINTS WITH THE CURRENT POS
    startX = event.offsetX;
    startY = event.offsetY;
    console.log("startX: " + startX);
    console.log("startY: " + startY);
});

