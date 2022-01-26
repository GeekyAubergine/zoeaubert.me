const CONTAINER_ID = 'hex-background'
const CANVAS_CLASS = 'hex-background'
const HIDDEN_CLASS = 'hex-background-hidden'

const HEX_POINTS = 6
const HEX_ANGLE = (Math.PI * 2) / HEX_POINTS
const HEX_RADIUS = 64
const HEX_PATH_WIDTH = 6
const HEX_BORDER_color = '#fac800'

let container = null
let canvas = null
let canvasContext

const gencolor = () => {
    const h = `${100 * (Math.random() * .04 + .48)}`
    const l = `${100 * (Math.random() * .06 + .60)}%`

    return `hsl(${h}, 100%, ${l})`
}

const drawHex = (cx, cy) => {
    canvasContext.beginPath()

    canvasContext.fillStyle = gencolor()
    canvasContext.strokeStyle = HEX_BORDER_color
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
    const width = container.parentNode.offsetWidth
    const height = container.parentNode.offsetHeight
    canvas.width  = width
    canvas.height = height

    drawGrid(width, height)

    console.timeEnd('render')

    setTimeout(() => {
        canvas.classList.remove(HIDDEN_CLASS)
    }, 10)
}

const onResize = _.throttle(render, 50)

const init = () => {
    container = document.querySelector(`#${CONTAINER_ID}`)

    console.log(container, container.parentNode.clientWidth)

    if (container == null) {
        return
    }

    window.onresize = onResize

    canvas = document.createElement('canvas')
    canvas.classList.add(CANVAS_CLASS)
    canvas.classList.add(HIDDEN_CLASS)
    container.appendChild(canvas)

    canvasContext = canvas.getContext('2d')

    render()
}

window.onload = init