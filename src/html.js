import React from 'react'
import PropTypes from 'prop-types'

const ALLOWED_STYLE_KEYS = [
    'gatsby-image-style',
    'gatsby-remark-autolink-headers-style',
]

export default function HTML(props) {
    // console.log({ h: props.headComponents })

    // Tailwind adds a huge amount of CSS to the head, which we don't need
    const headComponents = props.headComponents.filter(
        (component) =>
            component != null &&
            (component.type !== 'style' ||
                ALLOWED_STYLE_KEYS.includes(component.key)),
    )

    // const styleComponents = props.headComponents.filter(
    //     (component) =>
    //         component.type === 'style' &&
    //         !ALLOWED_STYLE_KEYS.includes(component.key),
    // )

    // console.log('style components')
    // styleComponents.forEach((component) => {
    //     console.log({ component })
    // })

    return (
        <html lang="en">
            <head>
                <meta charSet="utf-8" />
                <meta httpEquiv="x-ua-compatible" content="ie=edge" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, shrink-to-fit=no"
                />
                {props.headComponents}
            </head>
            <body className="hidden" {...props.bodyAttributes}>
                {props.preBodyComponents}
                <div
                    key={`body`}
                    id="___gatsby"
                    dangerouslySetInnerHTML={{ __html: props.body }}
                />
                {props.postBodyComponents}
            </body>
        </html>
    )
}
