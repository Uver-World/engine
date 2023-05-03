Neutralino.init();

import { square, canvas, context, drawSquare } from "./vars.js";
import { openSquareContextMenu } from "./openSquare.js";
import { openFile } from "./openFile.js";
let squares = [];

canvas.addEventListener("mousedown", (event) => {
    let mouseX = event.clientX - canvas.offsetLeft;
    let mouseY = event.clientY - canvas.offsetTop;
    for (let i = squares.length - 1; i >= 0; i--) {
        if (mouseX >= squares[i].x && mouseX <= squares[i].x + 100 && mouseY >= squares[i].y && mouseY <= squares[i].y + 100) {
            if (!squares[i].isDragable) {
                openSquareContextMenu(squares[i]);
                break;
            }
            squares[i].dragging = true;
            squares[i].dragStartX = mouseX - squares[i].x;
            squares[i].dragStartY = mouseY - squares[i].y;
            break;
        }
    }
});

canvas.addEventListener("mousemove", (event) => {
    for (let i = squares.length - 1; i >= 0; i--) {
        if (squares[i].dragging) {
            let mouseX = event.clientX - canvas.offsetLeft;
            let mouseY = event.clientY - canvas.offsetTop;
            squares[i].x = mouseX - squares[i].dragStartX;
            squares[i].y = mouseY - squares[i].dragStartY;
            context.clearRect(0, 0, canvas.width, canvas.height);
            for (let j = 0; j < squares.length; j++) {
                drawSquare(squares[j].x, squares[j].y, squares[j].fillStyle);
            }
            break;
        }
    }
});

canvas.addEventListener("mouseup", (event) => {
    for (let i = squares.length - 1; i >= 0; i--) {
        if (squares[i].dragging) {
            squares[i].dragging = false;
            let newSquare = Object.assign({}, square);;
            squares.push(newSquare);
            drawSquare(newSquare.x, newSquare.y, square.fillStyle);
            squares[i].isDragable = false;
            break;
        }
    }
});

Neutralino.window.setTitle("Ma fenêtre NeutralinoJS");
Neutralino.window.setSize(300, 300);
Neutralino.events.on("load", async () => {
    try {
        let fileId = await Neutralino.filesystem.openFile('./resources/saves/squares.json');
        let content = await Neutralino.filesystem.readFile('./resources/saves/squares.json');
        squares = JSON.parse(content);
        for (let i = 0; i < squares.length; i++) {
            drawSquare(squares[i].x, squares[i].y, squares[i].fillStyle);
        }
        return;
    } catch (e) {
        console.error(e);
    }
    let newSquare = Object.assign({}, square);;
    squares.push(newSquare)
    drawSquare(newSquare.x, newSquare.y, square.fillStyle);
});
Neutralino.events.on('windowClose', () => {
    Neutralino.app.exit();
});

const save_button = document.getElementById("save-button");
const load_button = document.getElementById("load-button");

save_button.addEventListener("click", (event) => {
    event.preventDefault();
    let data = JSON.stringify(squares);
    Neutralino.filesystem.writeFile('./resources/saves/squares.json', data);
});

load_button.addEventListener("click", async (event) => {
    event.preventDefault();
    const filename = await openFile();
    const data = await Neutralino.filesystem.readFile(filename);
    squares = JSON.parse(data);
    context.clearRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < squares.length; i++) {
        drawSquare(squares[i].x, squares[i].y, squares[i].fillStyle);
    }
});
// getUsername();
