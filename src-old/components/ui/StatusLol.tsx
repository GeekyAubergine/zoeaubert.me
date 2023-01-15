import React, { useRef } from 'react'

const STATUS_LOL_SRC = 'https://status.lol/geekyaubergine.js?time&link&fluent'

export default function StatusLol() {
    let ref = useRef<HTMLDivElement>(null)
    const [loaded, setLoaded] = React.useState(false)
    React.useEffect(() => {
        if (
            document &&
            document.querySelector(`script[src="${STATUS_LOL_SRC}"]`) == null &&
            document.querySelector('.statuslol_container') == null &&
            ref.current
        ) {
            const script = document.createElement('script')
            script.src = STATUS_LOL_SRC
            script.async = true

            ref.current.appendChild(script)

            script.onload = () => {
                setLoaded(true)
            }

            return () => {
                ref?.current?.removeChild(script)
            }
        }
    }, [])

    return (
        <div>
            <div className="flex h-16 items-center">
                {!loaded && <p className="text">Loading latest status</p>}
                <div ref={ref} className="h-16"></div>
            </div>
            <p className="secondary">
                See previous status' at{' '}
                <a
                    href="https://geekyaubergine.status.lol"
                    target="_blank"
                    rel="noopener"
                    className="secondary"
                >
                    status.lol
                </a>
            </p>
        </div>
    )
}
