import * as React from 'react'

export type Dimension = {
    width: number
    height: number
}

const getDimension = (element: HTMLElement | null): Dimension => {
    if (element == null) {
        return {
            width: 0,
            height: 0,
        }
    }

    return {
        width: element.offsetWidth,
        height: element.offsetHeight,
    }
}

export const useDimension = (ref: React.RefObject<HTMLElement>): Dimension => {
    const [dimension, setDimension] = React.useState(getDimension(ref.current))

    const resizeCallback = React.useCallback(() => {
        if (ref.current) {
            setDimension(getDimension(ref.current))
        }
    }, [ref])

    React.useLayoutEffect(() => {
        if (ref.current == null) {
            return
        }

        resizeCallback()

        window.addEventListener('resize', resizeCallback)

        return () => {
            window.removeEventListener('resize', resizeCallback)
        }
    }, [ref])

    return dimension
}
