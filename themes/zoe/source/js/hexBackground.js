const HEX_POINTS = 6
const HEX_ANGLE = (Math.PI * 2) / HEX_POINTS
const HEX_RADIUS = 64
const HEX_PATH_WIDTH = 6
const HEX_BORDER_COLOUR = '#ffd400'

let canvas = null
let canvasContext

function shadeColor2(color, percent) {
    var f=parseInt(color.slice(1),16),t=percent<0?0:255,p=percent<0?percent*-1:percent,R=f>>16,G=f>>8&0x00FF,B=f&0x0000FF;
    return "#"+(0x1000000+(Math.round((t-R)*p)+R)*0x10000+(Math.round((t-G)*p)+G)*0x100+(Math.round((t-B)*p)+B)).toString(16).slice(1);
}

function shadeRGBColor(color, percent) {
    var f=color.split(","),t=percent<0?0:255,p=percent<0?percent*-1:percent,R=parseInt(f[0].slice(4)),G=parseInt(f[1]),B=parseInt(f[2]);
    return "rgb("+(Math.round((t-R)*p)+R)+","+(Math.round((t-G)*p)+G)+","+(Math.round((t-B)*p)+B)+")";
}

const drawHex = (cx, cy) => {
    canvasContext.beginPath()

    canvasContext.fillStyle = `hsl(49.9,100%,${100 * (Math.random() * .08 + .6)}%)`
    canvasContext.strokeStyle = HEX_BORDER_COLOUR
    canvasContext.lineWidth = HEX_PATH_WIDTH

    for (let p = 0; p < HEX_POINTS; p++) {
        const x = cx + Math.sin(HEX_ANGLE * p) * HEX_RADIUS
        const y = cy + Math.cos(HEX_ANGLE * p) * HEX_RADIUS
        if (p === 0) {
            canvasContext.moveTo(x, y)
        } else {
            canvasContext.lineTo(x, y)
        }
    }

    canvasContext.closePath()
    canvasContext.fill()
    canvasContext.stroke()
}

const drawRow = (cx, cy, row, width) => {
    let count = Math.ceil(width / HEX_RADIUS)

    if (Math.abs(row) % 2 === 1) {
        count++
    }

    count = Math.floor(count / 2)

    const hexWidth = Math.sqrt(3) * HEX_RADIUS

    for (let h = -count; h <= count; h++) {
        let x = cx + hexWidth * h

        if (Math.abs(row) % 2 === 1) {
            x -= hexWidth / 2
        }

        drawHex(x, cy)
    }
}

const drawGrid = (width, height) => {
    const hexHeight = HEX_RADIUS * 1.5
    let rowCount = Math.ceil(height / hexHeight)

    rowCount = Math.floor(rowCount / 2)
    for (let row = -rowCount; row <= rowCount; row++) {
        const y = (height / 2) + (row * hexHeight)
        drawRow(width / 2, y, row, width)
    }
}

const render = () => {
    console.time('render')
    const width = window.innerWidth
    const height = window.innerHeight
    canvas.width  = width
    canvas.height = height

    drawGrid(width, height)


    // drawRow(width / 2, height / 2, 0, width)
    //
    // drawRow(width / 2, height / 2 + HEX_RADIUS * 1.5, 1, width)

    // drawHex(width / 2 + Math.sqrt(3) * HEX_RADIUS, height / 2 + HEX_RADIUS * .75)
    console.timeEnd('render')

    setTimeout(() => {
        canvas.classList.remove('backgroundHidden')
    }, 10)
}

const onResize = _.throttle(render, 50)

const init = () => {
    const container = document.querySelector('#background')

    canvas = document.createElement('canvas')
    canvas.classList.add('background')
    canvas.classList.add('backgroundHidden')
    container.appendChild(canvas)

    canvasContext = canvas.getContext('2d')

    render()
}

window.onload = init
window.onresize = onResize