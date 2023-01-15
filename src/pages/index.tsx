import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../components/ui/Page'
import BlogListItem from '../components/ui/BlogListItem'
import StatusLol from '../components/ui/StatusLol'
import MicroBlogPosts from '../components/ui/MicroBlogPosts'
import SEO from '../components/Seo'

function Heading({
    title,
    className = '',
}: {
    title: string
    className?: string
}) {
    return (
        <h2
            className={`text-lg mt-6 font-bold sm:mt-10 sm:mb-0 sm:text-2xl ${className}`}
        >
            {title}
        </h2>
    )
}

function SocialLink({
    name,
    description,
    link,
    rel = '',
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
        return (
            <BlogListItem
                node={node.childMarkdownRemark}
                key={node.childMarkdownRemark.id}
            />
        )
    }, [])

    return (
        <Page>
            <p className="text mt-4 leading-[1.6rem] sm:mt-6 sm:leading-6">
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
                and other projects, primarily focusing on app development.
            </p>
            <a
                href="https://geekyaubergine.status.lol"
                target="_blank"
                rel="noopener"
                className="link"
            >
                <Heading title="Status" />
            </a>
            <StatusLol />
            <Link to="/blog" className="text-lg link font-normal pb-1">
                <Heading title="Big Blogs" />
            </Link>
            {data.blogPosts.edges.map(renderBlogEntry)}
            <a
                href="https://geekyaubergine.com"
                target="_blank"
                rel="noopener"
                className="link"
            >
                <Heading title="Micro Blogs" />
            </a>
            <MicroBlogPosts />
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
                    description="Cross-posts and occasional funnies"
                    link="https://social.lol/@geekyaubergine"
                    rel="me"
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
        blogPosts: allFile(
            filter: { sourceInstanceName: { eq: "posts" } }
            limit: 5
            sort: { childMarkdownRemark: { frontmatter: { date: DESC } } }
        ) {
            edges {
                node {
                    id
                    childMarkdownRemark {
                        frontmatter {
                            title
                            slug
                            description
                            date(formatString: "YYYY-MM-DD")
                        }
                        id
                        timeToRead
                    }
                }
            }
        }
    }
`

export const Head = () => <SEO />
