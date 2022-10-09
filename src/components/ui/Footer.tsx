import { Link } from 'gatsby'
import * as React from 'react'

export default function Footer() {
    return (
        <footer className="flex w-full flex-ro justify-center border-t border-t-border mt-12 p-4 mx-auto">
            <Link to="/referals" className="link text-center">
                Referals
            </Link>
            <span className="text mx-2">-</span>
            <a
                className="link text-center"
                href="https://zoeaubert.me/rss.xml"
                target="_blank"
                rel="noopener"
            >
                RSS
            </a>
        </footer>
    )
}
