let square = { x: 50, y: 50, dragging: false, dragStartX: 0, dragStartY: 0, isDragable: true, fillStyle: "#FF0000" };
const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

let drawSquare = (x, y, fillStyle) => {
    context.fillStyle = fillStyle;
    context.fillRect(x, y, 100, 100);
}

export { square, canvas, context, drawSquare };