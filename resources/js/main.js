Neutralino.init();

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

let square = { x: 50, y: 50, dragging: false, dragStartX: 0, dragStartY: 0 };
let squares = [];

function drawSquare(x, y) {
    context.fillStyle = "#FF0000";
    context.fillRect(x, y, 100, 100);
}

canvas.addEventListener("mousedown", function (event) {
    let mouseX = event.clientX - canvas.offsetLeft;
    let mouseY = event.clientY - canvas.offsetTop;
    for (let i = squares.length - 1; i >= 0; i--) {
        console.log("mouseX: " + mouseX + " mouseY: " + mouseY);
        console.log("x: " + squares[i].x + " y: " + squares[i].y);
        if (mouseX >= squares[i].x && mouseX <= squares[i].x + 100 && mouseY >= squares[i].y && mouseY <= squares[i].y + 100) {
            squares[i].dragging = true;
            console.log("dragging");
            squares[i].dragStartX = mouseX - squares[i].x;
            squares[i].dragStartY = mouseY - squares[i].y;
            break;
        }
    }
});

canvas.addEventListener("mousemove", function (event) {
    for (let i = squares.length - 1; i >= 0; i--) {
        if (squares[i].dragging) {
            let mouseX = event.clientX - canvas.offsetLeft;
            let mouseY = event.clientY - canvas.offsetTop;
            squares[i].x = mouseX - squares[i].dragStartX;
            squares[i].y = mouseY - squares[i].dragStartY;
            context.clearRect(0, 0, canvas.width, canvas.height);
            for (let j = 0; j < squares.length; j++) {
                drawSquare(squares[j].x, squares[j].y);
            }
            break;
        }
    }
});

canvas.addEventListener("mouseup", function (event) {
    for (let i = squares.length - 1; i >= 0; i--) {
        if (squares[i].dragging) {
            squares[i].dragging = false;
            let newSquare = Object.assign({}, square);;
            squares.push(newSquare);
            drawSquare(newSquare.x, newSquare.y);
            break;
        }
    }
});

Neutralino.window.setTitle("Ma fenêtre NeutralinoJS");
Neutralino.window.setSize(300, 300);
Neutralino.events.on("load", function () {
    let newSquare = Object.assign({}, square);;
    squares.push(newSquare)
    drawSquare(newSquare.x, newSquare.y);
});
Neutralino.events.on('windowClose', () => {
    Neutralino.app.exit();
});
// getUsername();