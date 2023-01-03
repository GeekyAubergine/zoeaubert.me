import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../components/ui/Page'
import BlogListItem from '../components/ui/BlogListItem'

function Heading({ title }: { title: string }) {
    return (
        <h2 className="text-lg mt-6 font-bold sm:mt-6 sm:mb-0 sm:text-xl">
            {title}
        </h2>
    )
}

function SocialLink({
    name,
    description,
    link,
    rel = ''
}: {
    name: string
    description: string
    link: string
    rel?: string
}) {
    return (
        <div className="flex mb-3 last-of-type:mb-0">
            <a
                className="link text-x"
                href={link}
                target="_blank"
                rel={`noopener ${rel}`}
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
            <p className="text mt-2 leading-6">
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
            <Heading title="Blog Posts" />
            <p className="text mt-2 mb-2 leading-6">
                I put small posts on my{' '}
                <a
                    href="https://geekyaubergine.com"
                    target="_blank"
                    rel="noopener"
                >
                    Micro.blog
                </a>
            </p>
            {data.blogPosts.edges.map(renderBlogEntry)}
            <div className="mt-2">
                <Link to="/blog" className="text-lg link font-normal pb-1">
                    See More
                </Link>
            </div>
            <Heading title="Socials" />
            <div className="pt-4">
                <SocialLink
                    name="Micro.blog"
                    description="Replacement for Twitter and
                        Instagram"
                    link="https://geekyaubergine.com"
                />
                <SocialLink
                    name="Mastodon"
                    description="Replacement for Twitter"
                    link="https://social.lol/@geekyaubergine"
                    rel='me'
                />
                <SocialLink
                    name="GitHub"
                    description="My open source projects"
                    link="https://github.com/geekyaubergine"
                />
                <SocialLink
                    name="Twitter"
                    description="Old tweets and occasional retweets"
                    link="https://twitter.com/geekyaubergine"
                />
                <SocialLink
                    name="LinkedIn"
                    description="Professional things"
                    link="https://www.linkedin.com/in/zoeaubert/"
                />
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
