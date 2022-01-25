import {IGatsbyImageData} from "gatsby-plugin-image/dist/src/components/gatsby-image.browser";
import { Album } from "../res/photos/albumData";

export type MarkdownRemarkResponse = {
        frontmatter: {
            title: string,
            slug: string,
            tags: string[],
            description: string,
            date: string,
        },
        timeToRead: number,
        html: string,
}

export type MarkdownRemarkNode = {
    node: MarkdownRemarkResponse,
}

export type PhotoFile = {
    childImageSharp: {
        gatsbyImageData: IGatsbyImageData,
    }
}

export type PhotoResponse = {
    uid: string,
    alt: string,
    thumb: PhotoFile,
    full?: PhotoFile,
}

export type PhotoNode = {
    node: PhotoResponse,
}

export type AlbumResponse = Album

export type AlbumNode = {
    node: AlbumResponse,
}