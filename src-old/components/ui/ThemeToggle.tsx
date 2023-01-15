import * as React from 'react'
import { Link } from 'gatsby'

export function unhideBody() {
    if (typeof window !== 'undefined' && window.document) {
        requestAnimationFrame(() => {
            document.body.classList.remove('hidden')
        })
    }
}

export default function ThemeToggle() {
    React.useEffect(() => {
        if (typeof window !== 'undefined' && window.document) {
            const bodyClassList = document.body.classList

            const htmlClassList = document.querySelector('html')?.classList
            const toggleButton = document.querySelector('.toggle-button')
            const systemDarkSetting = window.matchMedia(
                '(prefers-color-scheme: dark)',
            )
            const storeDarkValue = localStorage.getItem('dark')

            const setDark = (isDark) => {
                if (!htmlClassList) {
                    return
                }
                htmlClassList[isDark ? 'add' : 'remove']('dark')
                localStorage.setItem('dark', isDark ? 'yes' : 'no')
            }

            setDark(
                storeDarkValue
                    ? storeDarkValue === 'yes'
                    : systemDarkSetting.matches,
            )

            requestAnimationFrame(() => {
                bodyClassList.remove('hidden')
            })

            toggleButton?.addEventListener('click', () => {
                if (!htmlClassList) {
                    return
                }
                setDark(!htmlClassList.contains('dark'))
            })
            systemDarkSetting.addEventListener('change', (event) =>
                setDark(event.matches),
            )
        }
    }, [])

    return (
        <div className="toggle-button ml-4 small:mx-4">
            <svg
                className="toggle-button cursor-pointer text-middleGray rotate-180 dark:rotate-0 transition-all duration-500"
                width="24"
                height="24"
                viewBox="0 0 100 100"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
            >
                <path className="fill-current" d="M 50 0 A 1 1 0 0 0 50 100" />
                <circle
                    cx="50"
                    cy="50"
                    r="44"
                    className="stroke-current"
                    strokeWidth="8"
                />
            </svg>
        </div>
    )
}
