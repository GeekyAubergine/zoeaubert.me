use hypertext::prelude::*;
use hypertext::Raw;
use maud::PreEscaped;

use crate::domain::models::albums::album_photo::AlbumPhoto;

pub fn home_page_scripts(silly_names: &[String]) -> impl Renderable {
    let silly_names_as_string = silly_names
        .iter()
        .map(|n| format!("'{}'", n))
        .collect::<Vec<String>>()
        .join(", ");

    let name_script = format!(
        r#"
        <script type="text/javascript">
            const TIME_BETWEEN_NAME_CHANGES = 5000;

            const TYPING_DELAY_MAX = 200;
            const TYPING_DELAY_MIN = 80;

            const nameElement = document.querySelector('.typing-name');
            const cursorElement = document.querySelector('.typing-cursor');

            // Cursor, split first and last names

            const names = [{}]
            let memory = ['Zoe Aubert'];

            const MEMORY_SIZE = Math.floor(names.length / 2);

            function typingDelay() {{
                return Math.floor(Math.random() * (TYPING_DELAY_MAX - TYPING_DELAY_MIN)) + TYPING_DELAY_MIN;
            }}

            function pickNewName() {{
                let next = names[Math.floor(Math.random() * names.length)];

                while (memory.includes(next)) {{
                    next = names[Math.floor(Math.random() * names.length)];
                }}

                memory.push(next);

                memory = memory.slice(-MEMORY_SIZE);

                return next;
            }}

            async function typeName(nextName) {{
                if (!nameElement) {{
                    return
                }}

                while (nameElement.innerHTML.length > 0) {{
                    nameElement.innerHTML = nameElement
                        .innerHTML
                        .substring(0, nameElement.innerHTML.length - 1);
                    await new Promise(resolve => setTimeout(resolve, typingDelay()));
                }}

                await new Promise(resolve => setTimeout(resolve, 500));

                for (let i = 0; i < nextName.length; i++) {{
                    nameElement.innerHTML = nextName.substring(0, i + 1);

                    await new Promise(resolve => setTimeout(resolve, typingDelay()));
                }}
            }}

            async function changeName() {{
                const nextName = pickNewName();

                await typeName(nextName);

                setTimeout(changeName, TIME_BETWEEN_NAME_CHANGES);
            }}

            async function main() {{
                if (nameElement && cursorElement) {{
                    await new Promise(resolve => setTimeout(resolve, 2000));

                    cursorElement
                        .classList
                        .remove('!opacity-0')

                    setTimeout(changeName, 1500);
                }}
            }}

            main();
        </script>
        "#,
        silly_names_as_string
    );

    maud! {
        (Raw::dangerously_create(&name_script))
    }
}

pub fn album_photo_controls_scripts(photo: &AlbumPhoto, previous_photo: Option<&AlbumPhoto>, next_photo: Option<&AlbumPhoto>) -> impl Renderable {
    let previous_photo_function = match previous_photo {
        Some(previous_photo) => {
            format!(
                r#"
                function goToPreviousPhoto() {{
                    window
                        .history
                        .replaceState(null, '', '{}')
                    location.reload()
                }}
                "#,  previous_photo.slug.relative_string() )
        },
        None => "".to_string()
    };

    let next_photo_function = match next_photo {
        Some(next_photo) => {
            format!(
                r#"
                function goToNextPhoto() {{
                    window
                        .history
                        .replaceState(null, '', '{}')
                    location.reload()
                }}
                "#,  next_photo.slug.relative_string() )
        },
        None => "".to_string()
    };

    let controls_script = format!(
        r#"
        <script type="text/javascript">
            {}
            {}

            function onKeyUp(event) {{
                if (event.key === 'Escape') {{
                    window
                        .history
                        .back()
                }}
                if (event.key === 'ArrowLeft') {{
                    goToPreviousPhoto?.()
                }}
                if (event.key === 'ArrowRight') {{
                    goToNextPhoto?.()
                }}
            }}

            window.addEventListener('keyup', onKeyUp)
        </script>
        "#,
        previous_photo_function,
        next_photo_function
    );

    maud! {
        (Raw::dangerously_create(&controls_script))
    }
}
