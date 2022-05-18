import * as React from 'react'
import { Link } from 'gatsby'

export default function NavBar() {
    return (
        <nav className="flex justify-between items-baseline mt-2">
            <h1 className={`text-4xl text-center`}>
                Zoe Aubert
            </h1>
            <div>
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
