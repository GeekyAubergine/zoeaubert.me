import * as React from 'react'
import { Link } from 'gatsby'

export default function NavBar() {
    const bodyClassList = document.body.classList
    const htmlClassList = document.querySelector('html').classList
    const toggleButton = document.querySelector('.toggle-button')
    const systemDarkSetting = window.matchMedia('(prefers-color-scheme: dark)')
    const storeDarkValue = localStorage.getItem('dark')

    const setDark = React.useCallback(
        (isDark) => {
            htmlClassList[isDark ? 'add' : 'remove']('dark')
            localStorage.setItem('dark', isDark ? 'yes' : 'no')
        },
        [htmlClassList],
    )

    const onTogglePressed = React.useCallback(() => {
        setDark(!htmlClassList.contains('dark'))
    }, [setDark, htmlClassList])

    React.useEffect(() => {
        setDark(
            storeDarkValue
                ? storeDarkValue === 'yes'
                : systemDarkSetting.matches,
        )
        requestAnimationFrame(() => bodyClassList.remove('hidden'))

        systemDarkSetting.addEventListener('change', (event) =>
            setDark(event.matches),
        )
    }, [])

    return (
        <nav className="flex flex-1 flex-col justify-center items-center my-2 sm:flex-row sm:justify-between">
            <div className="w-full flex flex-row items-center justify-between mb-4 sm:justify-start">
                <Link
                    className="text-3xl h-full text-center sm:text-left sm:text-5xl"
                    to="/"
                >
                    <h1>Zoe Aubert</h1>
                </Link>
                <div className="mx-4" onClick={onTogglePressed}>
                    <svg
                        className="toggle-button cursor-pointer text-middleGray dark:rotate-180 transition-all duration-500"
                        width="24"
                        height="24"
                        viewBox="0 0 100 100"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                    >
                        <path
                            className="fill-current"
                            d="M 50 0 A 1 1 0 0 0 50 100"
                        />
                        <circle
                            cx="50"
                            cy="50"
                            r="44"
                            className="stroke-current"
                            stroke-width="8"
                        />
                    </svg>
                </div>
            </div>
            <div className="flex w-full justify-around sm:justify-end">
                <a
                    className="navbarLink"
                    href="https://micro.zoeaubert.me"
                    target="_blank"
                    rel="noopener"
                >
                    MicroBlog
                </a>
                <Link className="navbarLink" to="/blog">
                    Blog
                </Link>
                <a
                    className="navbarLink"
                    href="https://micro.zoeaubert.me/photos"
                    target="_blank"
                    rel="noopener"
                >
                    Photos
                </a>
                <a
                    className="navbarLink"
                    href="https://github.com/geekyaubergine"
                    target="_blank"
                    rel="noopener"
                >
                    GitHub
                </a>
            </div>
        </nav>
    )
}
