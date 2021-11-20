import * as React from 'react'
import * as styles from './header.module.scss'
import {Link} from "gatsby";

const Header = () => (
    <div className={styles.container}>
        <Link className={styles.titleContainer} to="/">
            <h1 className={styles.title}>Zoe Aubert</h1>
        </Link>
        <div className={styles.links}>
            <Link className={styles.link} to="/photos">Photos</Link>
            <Link className={styles.link} to="/blog">Blog</Link>
        </div>
    </div>
)

export default Header
