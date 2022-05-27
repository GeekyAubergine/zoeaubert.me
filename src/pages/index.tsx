import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../components/ui/Page'
import BlogListItem from '../components/ui/BlogListItem'

function Heading({ title }: { title: string }) {
    return <h2 className="text-3xl pt-12 font-bold sm:pt-8">{title}</h2>
}

function SocialLink({
    name,
    description,
}: {
    name: string
    description: string
}) {
    return (
        <div className="flex text-xl mt-2 last-of-type:mb-0">
            <a
                className="link text-x"
                href="https://micro.blog/geekyaubergine"
                target="_blank"
                rel="noopener"
            >
                {name}
            </a>
            <span className="text pl-1 sm:pl-2">-</span>
            <span className="text pl-1 sm:pl-2">{description}</span>
        </div>
    )
}

export default function IndexPage({ data }) {
    const renderBlogEntry = React.useCallback(({ node }) => {
        return <BlogListItem node={node} key={node.id} style=""  />
    }, [])

    return (
        <Page>
            <p className="text-xl pt-4">
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
            <p className="text-xl pt-4">
                I put small posts and photos on my{' '}
                <a
                    href="https://micro.zoeaubert.me"
                    target="_blank"
                    rel="noopener"
                >
                    Micro.blog
                </a>{' '}
                and longer writings here.
            </p>
            <Heading title="Blog Posts" />
            {data.blogPosts.edges.map(renderBlogEntry)}
            <div className="mt-2">
                <Link to="/blog" className="button mb-0 mt-2">
                    See More
                </Link>
            </div>
            <Heading title="Other Platforms" />
            <div className="sm:pt-2">
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
                <SocialLink name="LinkedIn" description="Professional things" />
            </div>
        </Page>
    )
}

export const pageQuery = graphql`
    {
        blogPosts: allMarkdownRemark(
            sort: { order: DESC, fields: [frontmatter___date] }
            limit: 3
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
