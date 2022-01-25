import * as React from 'react'
import * as styles from './header.module.scss'
import { Link } from 'gatsby'

const Header = () => (
    <div className={styles.container}>
        <Link className={styles.titleContainer} to="/">
            <h1 className={styles.title}>Zoe Aubert</h1>
        </Link>
        <div className={styles.links}>
            <Link
                className={styles.link}
                activeClassName={styles.linkActive}
                to="/blog"
            >
                Blog
            </Link>
            <a
                className={styles.link}
                href="https://github.com/geekyaubergine"
                target="_blank"
                rel="noopener"
            >
                GitHub
            </a>
            <a
                className={styles.link}
                href="https://twitter.com/geekyaubergine"
                target="_blank"
                rel="noopener"
            >
                Twitter
            </a>
            <a
                className={styles.link}
                href="https://www.linkedin.com/in/zoeaubert/"
                target="_blank"
                rel="noopener"
            >
                LinkedIn
            </a>
            {/* <Link className={styles.link} to="/photos">Photos</Link> */}
        </div>
    </div>
)

export default Header
