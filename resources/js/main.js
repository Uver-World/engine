Neutralino.init();

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

let squareX = 50;
let squareY = 50;
let dragging = false;
let dragStartX;
let dragStartY;

function drawSquare() {
    context.fillStyle = "#FF0000";
    context.fillRect(squareX, squareY, 100, 100);
}

canvas.addEventListener("mousedown", function (event) {
    let mouseX = event.clientX - canvas.offsetLeft;
    let mouseY = event.clientY - canvas.offsetTop;
    if (mouseX >= squareX && mouseX <= squareX + 100 && mouseY >= squareY && mouseY <= squareY + 100) {
        dragging = true;
        dragStartX = mouseX - squareX;
        dragStartY = mouseY - squareY;
    }
});

canvas.addEventListener("mousemove", function (event) {
    if (dragging) {
        let mouseX = event.clientX - canvas.offsetLeft;
        let mouseY = event.clientY - canvas.offsetTop;
        squareX = mouseX - dragStartX;
        squareY = mouseY - dragStartY;
        context.clearRect(0, 0, canvas.width, canvas.height);
        drawSquare();
    }
});

canvas.addEventListener("mouseup", function (event) {
    dragging = false;
});

Neutralino.window.setTitle("Ma fenêtre NeutralinoJS");
Neutralino.window.setSize(300, 300);
Neutralino.events.on("load", function () {
    drawSquare();
});
Neutralino.events.on('windowClose', () => {
    Neutralino.app.exit();
});
// getUsername();