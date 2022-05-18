import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../components/ui/Page'
import NavBar from '../components/ui/NavBar'
import BlogListItem from '../components/ui/BlogListItem'

function Heading({ title }: { title: string }) {
    return <h2 className="text-2xl pt-8 font-bold">{title}</h2>
}

function SocialLink({
    name,
    description,
}: {
    name: string
    description: string
}) {
    return (
        <li className="flex text-xl ml-4 mb-2 last-of-type:mb-0">
            <a
                className="link text-x"
                href="https://micro.blog/geekyaubergine"
                target="_blank"
                rel="noopener"
            >
                {name}
            </a>
            <span className="text pl-2">-</span>
            <span className="text pl-2">{description}</span>
        </li>
    )
}

export default function IndexPage({ data }) {
    const renderBlogEntry = React.useCallback(({ node }) => {
        return <BlogListItem node={node} key={node.id} />
    }, [])

    return (
        <Page>
            <NavBar />
            <p className="text-xl pt-4 px-4">
                Hi there, I’m a software developer from Jersey, living in
                Portsmouth, working at{' '}
                <a href="https://radweb.co.uk" target="_blank" rel="noopener">
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
                and other projects; primarily focusing on app development.
            </p>
            <p className="text-xl pt-4 px-4">
                I tend to post small things on my{' '}
                <a
                    href="https://micro.zoeaubert.me"
                    target="_blank"
                    rel="noopener"
                >
                    micro blog
                </a>{' '}
                and longer writings on my <Link to="/blog">Blog</Link>. For
                photos see my{' '}
                <a
                    href="https://micro.zoeaubert.me/photos/"
                    target="_blank"
                    rel="noopener"
                >
                    photos page
                </a>
            </p>
            <Heading title="Blog Posts" />
            {data.blogPosts.edges.map(renderBlogEntry)}
            <Link to="/blog">
                <p className="m-4 mb-0 link">See More</p>
            </Link>
            <Heading title="Socials & Other Platforms" />
            <ul className="pt-4">
                <SocialLink
                    name="Micro.blog"
                    description="Micro blogging replacement for both Twitter and
                        Instagram"
                />
                <SocialLink
                    name="GitHub"
                    description="My open source projects"
                />
                <SocialLink
                    name="Twitter"
                    description="Old tweets and occasional retweets of cool things"
                />
                <SocialLink
                    name="LinkedIn"
                    description="Professional things"
                />
            </ul>
        </Page>
    )
}

export const pageQuery = graphql`
    {
        blogPosts: allMarkdownRemark(
            sort: { order: DESC, fields: [frontmatter___date] }
            limit: 5
            filter: { fileAbsolutePath: { regex: "/res/blog_posts/" } }
        ) {
            pageInfo {
                perPage
            }
            edges {
                node {
                    id
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
