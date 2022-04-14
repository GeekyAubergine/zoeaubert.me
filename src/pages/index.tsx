import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { TEXT as BASE_STYLE_TEXT } from '../tailwind-base-styles'
import { Page } from '../components/ui/Page'
import { faCoffee } from '@fortawesome/free-solid-svg-icons'
import {
    faGithub,
    faLinkedin,
    faLinkedinIn,
    faTwitter,
} from '@fortawesome/free-brands-svg-icons'

const IndexPage = ({ data }) => {
    return (
        <Page>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                className="absolute w-0 h-0"
            >
                <defs>
                    <clipPath id="hexMask" clipPathUnits="objectBoundingBox">
                        {/* <path d="M 0.7215940906156465 0.023910587916932343 a 0.14608564688903067 0.14608564688903067 0 0 0.018260705861128834 0.14608564688903067 0 l 0.8660254037844372 0.5 a 0.14608564688903067 0.14608564688903067 0 0 0.018260705861128834 0.07304282344451533 0.12651388133418354 l 0 1 a 0.14608564688903067 0.14608564688903067 0 0 0.018260705861128834 -0.07304282344451533 0.12651388133418354 l -0.8660254037844372 0.5 a 0.14608564688903067 0.14608564688903067 0 0 0.018260705861128834 -0.14608564688903067 0 l -0.8660254037844372 -0.5 a 0.14608564688903067 0.14608564688903067 0 0 0.018260705861128834 -0.07304282344451533 -0.12651388133418354 l 0 -1 a 0.14608564688903067 0.14608564688903067 0 0 0.018260705861128834 0.07304282344451533 -0.12651388133418354 Z" /> */}
                        {/* <path d="M 39.516221119999976 1.3094010767584998
a 8 8 0 0 1 8 0
l 47.42562584220397 27.381197846482983
a 8 8 0 0 1 4 6.9282032302755
l 0 54.76239569296597
a 8 8 0 0 1 -4 6.9282032302755
l -47.42562584220397 27.381197846482983
a 8 8 0 0 1 -8 0
l -47.42562584220397 -27.381197846482983
a 8 8 0 0 1 -4 -6.9282032302755
l 0 -54.76239569296597
a 8 8 0 0 1 4 -6.9282032302755" /> */}
                        /** 
                         p.split(' ').map(s => /[^.0-9\-]/.test(s) ? s : (Number.parseFloat(s) / 128).toString()).join(' ')
                        M 0.3087204774999998 0.01022969591217578
a 0.0625 0.0625 0 0 0.0078125 0.0625 0
l 0.3705127018922185 0.2139156081756483
a 0.0625 0.0625 0 0 0.0078125 0.03125 0.05412658773652734
l 0 0.4278312163512966
a 0.0625 0.0625 0 0 0.0078125 -0.03125 0.05412658773652734
l -0.3705127018922185 0.2139156081756483
a 0.0625 0.0625 0 0 0.0078125 -0.0625 0
l -0.3705127018922185 -0.2139156081756483
a 0.0625 0.0625 0 0 0.0078125 -0.03125 -0.05412658773652734
l 0 -0.4278312163512966
a 0.0625 0.0625 0 0 0.0078125 0.03125 -0.05412658773652734 */
                        <path
                            d="
                            M 0.473 0.01
                            a 0.063 0.063 0 0 1 0.063 0
                            l 0.371 0.214
                            a 0.063 0.063 0 0 1 0.031 0.054
                            l 0 0.428
                            a 0.063 0.063 0 0 1 -0.031 0.054
                            l -0.371 0.214
                            a 0.063 0.063 0 0 1 -0.062 0
                            l -0.371 -0.214
                            a 0.063 0.063 0 0 1 -0.031 -0.054
                            l 0 -0.428
                            a 0.063 0.063 0 0 1 0.031 -0.054
                            Zå
                        "
                        />
                    </clipPath>
                </defs>
            </svg>
            <div className="relative h-screen2/3 w-full">
                <div className="absolute top-0 w-full h-screen2/3 overflow-hidden">
                    <svg
                        width="100%"
                        height="100%"
                        className="bg-background dark:bg-background-dark"
                    >
                        <defs>
                            <pattern
                                id="hexagons"
                                width="50"
                                height="43.4"
                                patternUnits="userSpaceOnUse"
                                patternTransform="scale(2) translate(0) rotate(0)"
                            >
                                <polygon
                                    points="24.8,22 37.3,29.2 37.3,43.7 24.8,50.9 12.3,43.7 12.3,29.2"
                                    id="hex"
                                    className="stroke-gray-400 dark:stroke-gray-900 fill-background dark:fill-background-dark"
                                />
                                <use href="#hex" x="25" />
                                <use href="#hex" x="-25" />
                                <use href="#hex" x="12.5" y="-21.7" />
                                <use href="#hex" x="-12.5" y="-21.7" />
                            </pattern>
                        </defs>
                        <path
                            fill="red"
                            transform="rotate(90)"
                            d="M60.5 2.8094010767585a8 8 0 0 1 8 0l47.425625842204 27.381197846483a8 8 0 0 1 4 6.9282032302755l0 54.762395692966a8 8 0 0 1 -4 6.9282032302755l-47.425625842204 27.381197846483a8 8 0 0 1 -8 0l-47.425625842204 -27.381197846483a8 8 0 0 1 -4 -6.9282032302755l0 -54.762395692966a8 8 0 0 1 4 -6.9282032302755"
                        />
                    </svg>
                </div>
                <div className="flex flex-col w-full justify-center items-center bg-red-400">
                    <div className="flex flex-col w-[70%] p-8 justify-center items-center backdrop-blur-sm bg-opacity-25 bg-background dark:bg-background-dark rounded-md">
                        <h1
                            className={`${BASE_STYLE_TEXT} text-6xl text-center`}
                        >
                            Zoe Aubert
                        </h1>

                        <p
                            className={`${BASE_STYLE_TEXT} text-xl text-center mt-[5%]`}
                        >
                            Hi there, I’m a software developer from Jersey,
                            living in Portsmouth, working at{' '}
                            <a
                                href="https://radweb.co.uk"
                                target="_blank"
                                rel="noopener"
                            >
                                Radweb
                            </a>{' '}
                            on{' '}
                            <a
                                href="https://inventorybase.co.uk"
                                target="_blank"
                                rel="noopener"
                            >
                                InventoryBase
                            </a>{' '}
                            and other projects; primarily focusing on app
                            development.
                        </p>

                        <div className="flex flex-row mt-[5%]">
                            <a
                                className="hexMask flex flex-col justify-center items-center mx-1 w-24 h-24  bg-[#333333] text-text-dark"
                                href="https://github.com/geekyaubergine"
                                target="_blank"
                                rel="noopener"
                            >
                                <FontAwesomeIcon
                                    icon={faGithub}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                GitHub
                            </a>
                            <a
                                className="hexMask flex flex-col justify-center items-center mx-1 w-24 h-24 bg-[#1DA1F2] text-text-dark"
                                href="https://twitter.com/geekyaubergine"
                                target="_blank"
                                rel="noopener"
                            >
                                <FontAwesomeIcon
                                    icon={faTwitter}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                Twitter
                            </a>
                            <a
                                className="hexMask flex flex-col justify-center items-center mx-1 w-24 h-24 bg-[#0A66C2] text-text-dark"
                                href="https://www.linkedin.com/in/zoeaubert/"
                                target="_blank"
                                rel="noopener"
                            >
                                <FontAwesomeIcon
                                    icon={faLinkedin}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                LinkedIn
                            </a>
                        </div>

                        <div className="flex flex-row mt-[5%]">
                            <a
                                className="flex flex-col justify-center items-center mx-1 w-24 aspect-square bg-[#333333] text-text-dark rounded-full"
                                href="https://github.com/geekyaubergine"
                                target="_blank"
                                rel="noopener"
                            >
                                <FontAwesomeIcon
                                    icon={faGithub}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                GitHub
                            </a>
                            <a
                                className="flex flex-col justify-center items-center mx-1 w-24 aspect-square bg-[#1DA1F2] rounded-full text-text-dark"
                                href="https://twitter.com/geekyaubergine"
                                target="_blank"
                                rel="noopener"
                            >
                                <FontAwesomeIcon
                                    icon={faTwitter}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                Twitter
                            </a>
                            <a
                                className="flex flex-col justify-center items-center mx-1 w-24 aspect-square bg-[#0A66C2] rounded-full text-text-dark"
                                href="https://www.linkedin.com/in/zoeaubert/"
                                target="_blank"
                                rel="noopener"
                            >
                                <FontAwesomeIcon
                                    icon={faLinkedin}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                LinkedIn
                            </a>
                        </div>

                        <div className="flex flex-row mt-[5%]">
                            <a
                                className="hex flex flex-col justify-center items-center mx-1 w-24 h-24 bg-[#333333] text-text-dark rounded"
                                href="https://github.com/geekyaubergine"
                                target="_blank"
                                rel="noopener"
                            >
                                <div className="hex-corner bg-[#2C2C2C] w-full h-full"></div>
                                <FontAwesomeIcon
                                    icon={faGithub}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                GitHub
                            </a>
                            <a
                                className="hex flex flex-col justify-center items-center mx-1 w-24 h-24 bg-[#1DA1F2] text-text-dark"
                                href="https://twitter.com/geekyaubergine"
                                target="_blank"
                                rel="noopener"
                            >
                                <div className="hex-corner bg-[#0D93E5] w-full h-full"></div>
                                <FontAwesomeIcon
                                    icon={faTwitter}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                Twitter
                            </a>
                            <a
                                className="hex flex flex-col justify-center items-center mx-1 w-24 h-24 bg-[#0A66C2] text-text-dark"
                                href="https://www.linkedin.com/in/zoeaubert/"
                                target="_blank"
                                rel="noopener"
                            >
                                <div className="hex-corner bg-[#095EB2] w-full h-full"></div>
                                <FontAwesomeIcon
                                    icon={faLinkedin}
                                    size="2x"
                                    className="text-text-dark p-0 m-0 w-8 h-8"
                                />
                                LinkedIn
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </Page>
    )
}

export const pageQuery = graphql`
    {
        allMarkdownRemark(
            sort: { order: DESC, fields: [frontmatter___date] }
            limit: 2
            filter: { fileAbsolutePath: { regex: "/res/blog_posts/" } }
        ) {
            pageInfo {
                perPage
            }
            edges {
                node {
                    frontmatter {
                        title
                        slug
                        tags
                        description
                        date(formatString: "YYYY-MM-DD")
                    }
                    timeToRead
                }
            }
        }
    }
`

export default IndexPage
