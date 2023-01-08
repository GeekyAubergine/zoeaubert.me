import * as React from 'react'
import { Link } from 'gatsby'
import ThemeToggle from './ThemeToggle'

export default function NavBar() {
    return (
        <nav className="flex flex-1 flex-col justify-center items-center mb-4 sm:flex-row sm:justify-between">
            <div className="flex w-full flex-row items-center justify-between my-2 sm:justify-start">
                <Link
                    className="text-4xl h-full text-left"
                    to="/"
                >
                    <h1>Zoe Aubert</h1>
                </Link>
                <ThemeToggle />
            </div>
            <div className="flex w-full justify-between mt-2 sm:justify-end">
                <a
                    className="navbarLink"
                    href="https://geekyaubergine.com"
                    target="_blank"
                    rel="noopener"
                >
                    MicroBlog
                </a>
                <Link className="navbarLink" to="/blog">
                    Blog
                </Link>
                <Link className="navbarLink" to="/photos">
                    Photos
                </Link>
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
