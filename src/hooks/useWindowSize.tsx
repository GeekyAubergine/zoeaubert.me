import * as React from 'react'
import type { Dimension } from './useDimension'

const getDimension = (): Dimension => ({
    width: window.innerWidth,
    height: window.innerHeight,
})

export const useWindowSize = () => {
    const [dimension, setDimension] = React.useState(getDimension())

    const resizeCallback = React.useCallback(() => {
        setDimension(getDimension())
    }, [])

    React.useLayoutEffect(() => {
        resizeCallback()

        window.addEventListener('resize', resizeCallback)

        return () => {
            window.removeEventListener('resize', resizeCallback)
        }
    }, [])

    return dimension
}
