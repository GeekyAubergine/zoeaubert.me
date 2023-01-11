import { IGatsbyImageData } from 'gatsby-plugin-image/dist/src/components/gatsby-image.browser'

export type MarkdownRemarkResponse = {
    frontmatter: {
        title: string
        slug: string
        tags: string[]
        description: string
        date: string
    }
    timeToRead: number
    html: string
}

export type MarkdownRemarkNode = {
    node: MarkdownRemarkResponse
}

export type Photo = {
    id: string
    url: string
    description: string
    tags: string[]
    featured: boolean
    localFile: {
        childImageSharp: {
            gatsbyImageData: IGatsbyImageData
            original: {
                width: number
                height: number
            }
        }
        publicURL
    }
}

export type Album = {
    uid: string
    title: string
    description: string
    date: string
    year: number
    photos: Photo[]
}
