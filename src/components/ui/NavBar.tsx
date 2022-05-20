import * as React from 'react'
import { Link } from 'gatsby'

export default function NavBar() {
    return (
        <nav className="flex flex-1 flex-col justify-center items-center my-2 sm:flex-row sm:justify-between">
            <Link
                className="w-full text-4xl h-full text-center mb-4 sm:text-left sm:mb-0 sm:text-5xl"
                to="/"
            >
                <h1>Zoe Aubert</h1>
            </Link>
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
