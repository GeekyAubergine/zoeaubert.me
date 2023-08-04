module.exports = {
    darkMode: 'class',
    content: ['./src/**/**/*.{njk,md}'],
    safelist: [
        'gatsby-highlight',
        'photogrid',
        'gatsby-resp-image-image',
        'image-wrapper',
        'statuslol_container',
        'statuslol',
        'statuslol_emoji_container',
        'statuslol_content',
        'statuslol_time',
        'statuslol_emoji',
        'col-span-1',
        'col-span-2',
        'col-span-3',
        'col-span-4',
        'col-span-5',
        'col-span-6',
        'col-span-7',
        'col-span-8',
        'col-span-9',
    ],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#181818',
                },
                headings: {
                    DEFAULT:'#080808',
                    dark: '#EDEDED',
                },
                text: {
                    DEFAULT:'#080808',
                    dark: '#EDEDED',
                },
                secondary: {
                    DEFAULT:'#55535A',
                    dark: '#ABA9B0',
                },
                accent: {
                    DEFAULT:'#842A87', //'#FEB847',//#F58123', //'#89BA6A', //'#FEB847', //'#DEB9FF', //'#BB9EE0',//'#FEB847', ///#FEB847",//"#F2DE7C",//"#FEB847",//D9BBFF//00D5C6
                    dark: '#ED95E6',
                },
                code: {
                    DEFAULT: '#D4D4D4',
                    dark: "#424546",
                },
                border: {
                    DEFAULT:'#55535A',
                    dark: '#181818',
                },
                middleGray: {
                    DEFAULT:'#888888',
                    dark: '#181818',
                },
            },
            // colors: {
            //     background: '#181818',
            //     headings: '#E4E3E7',
            //     text: '#EDEDED',
            //     secondary: '#ABA9B0',
            //     tag: '#181D27',
            //     accent: "#FFA0F8" ,//'#FFA0F8', //"BC8DFE" //'#FEB847',//#F58123', //'#89BA6A', //'#FEB847', //'#DEB9FF', //'#BB9EE0',//'#FEB847', ///#FEB847",//"#F2DE7C",//"#FEB847",//D9BBFF//00D5C6
            //     code: {
            //         DEFAULT: '#181D27',
            //         text: '#0061DF8EA45C',
            //     },
            //     border: '#514F55',
            //     middleGray: '#888888',
            // },
        },
        fontFamily: {
            sans: ['Helvetica', 'Arial', 'sans-serif'],
        },
    },
    plugins: [],
}
