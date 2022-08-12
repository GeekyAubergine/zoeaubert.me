import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../components/ui/Page'
import BlogListItem from '../components/ui/BlogListItem'

function Heading({ title }: { title: string }) {
    return <h2 className="text-lg mt-9 font-bold sm:mt-12 sm:mb-0 sm:text-xl">{title}</h2>
}

function SocialLink({
    name,
    description,
}: {
    name: string
    description: string
}) {
    return (
        <div className="flex mb-3 last-of-type:mb-0">
            <a
                className="link text-x"
                href="https://micro.blog/geekyaubergine"
                target="_blank"
                rel="noopener"
            >
                {name}
            </a>
            <span className="text ml-1 sm:ml-2">-</span>
            <span className="text ml-1 sm:ml-2">{description}</span>
        </div>
    )
}

export default function IndexPage({ data }) {
    const renderBlogEntry = React.useCallback(({ node }) => {
        return <BlogListItem node={node} key={node.id} />
    }, [])

    return (
        <Page>
            <p className="text-lg mt-2 leading-6">
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
            <p className="text-lg mt-4 leading-6">
                I put small posts on my{' '}
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
                <Link to="/blog" className="text-lg link font-normal pb-1">
                    See More
                </Link>
            </div>
            <Heading title="Other Platforms" />
            <div className="pt-4">
                <SocialLink
                    name="Micro.blog"
                    description="Replacement for Twitter and
                        Instagram"
                />
                <SocialLink
                    name="GitHub"
                    description="My open source projects"
                />
                <SocialLink
                    name="Twitter"
                    description="Old tweets and occasional retweets"
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
