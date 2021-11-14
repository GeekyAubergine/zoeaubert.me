module.exports = {
  siteMetadata: {
    siteUrl: "https://zoeaubert.me",
    title: "zoeaubert.me",
    description: "",
    author: "Zoe Aubert | GeekyAubergine",
  },
  plugins: [
    "gatsby-plugin-sass",
    "gatsby-plugin-image",
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-sitemap",
    "gatsby-plugin-robots-txt",
    "gatsby-plugin-mdx",
    "gatsby-plugin-sharp",
    {
      resolve: `gatsby-plugin-google-fonts`,
      options: {
        fonts: [
          `Nunito:300,300i,400,400i,500,500i,600,600i,700,700i`,
          `Poppins:300,300i,400,400i,500,500i,600,600i,700,700i`,
          `Muli:300,300i,400,400i,500,500i,600,600i,700,700i`,
        ],
        display: `swap`,
        subset: `greek-ext,latin-ext`,
      },
    },
    {
      resolve: "gatsby-plugin-manifest",
      options: {
        name: `gatsby-starter-default`,
        short_name: `starter`,
        start_url: `/`,
        background_color: `#ECBDF2`,
        theme_color: `#ECBDF2`,
        display: `minimal-ui`,
        icon: "./res/images/icon.png",
      },
    },
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "pages",
        path: "./src/pages/",
      },
      __key: "pages",
    },
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "images",
        path: "./res/images/",
      },
      __key: "images",
    },
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "blog_posts",
        path: "./res/blog_posts/",
      },
      __key: "blog_posts",
    },
    "gatsby-transformer-sharp",
    "gatsby-transformer-remark",
  ],
};
