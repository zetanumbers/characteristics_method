import { CurveView } from "../pkg/index";

async function main() {
    const { Renderer, UDiff, CurveView } = await import("../pkg/index");

    // Enable interface
    document.querySelectorAll("input, select, button").forEach(
        (v: HTMLInputElement | HTMLSelectElement) => v.disabled = false
    );

    const pauseButton = document.getElementById("pause_button") as HTMLInputElement;
    const bottomUInput = document.getElementById("bottom_u_input") as HTMLInputElement;
    const bottomUtInput = document.getElementById("bottom_u_t_input") as HTMLInputElement;
    const aInput = document.getElementById("a_input") as HTMLInputElement;
    const lInput = document.getElementById("l_input") as HTMLInputElement;

    // get required params per set button click
    const getSetParams = () => {
        const bottomUText = bottomUInput.value;
        const bottomUtText = bottomUtInput.value;
        return [
            eval("x => { return " + bottomUText + "; }"),
            eval("x => { return " + bottomUtText + "; }"),
            parseFloat(aInput.value),
            parseFloat(lInput.value),
        ] as [
                (x: number) => number,
                (x: number) => number,
                number,
                number
            ];
    };


    const getSideFunc = (side: "left" | "right") => {
        const text = (document.getElementById(side + "_fn_input") as HTMLInputElement).value;
        const ty = (document.getElementById(side + "_fn_select") as HTMLInputElement).value;
        return new UDiff(ty, eval("t => { return " + text + "; }"));
    };

    const getViewElems = (funcName: string) => {
        return [funcName + "_visible", funcName + "_color"].map(id =>
            document.getElementById(id)) as [HTMLInputElement, HTMLInputElement];
    };
    const curveElems = [getViewElems("u"), getViewElems("u_x"), getViewElems("u_t")];
    const getCurveViews = () => {
        return curveElems.map(
            ([checkbox, colorInput]) => new CurveView(checkbox.checked, colorInput.value)
        ) as [CurveView, CurveView, CurveView];
    };

    const renderer = new Renderer(getSideFunc("left"), getSideFunc("right"), ...getSetParams());

    const canvas_context = (document.getElementById("canvas") as HTMLCanvasElement).getContext("2d");
    renderer.render_canvas(canvas_context, ...getCurveViews());

    var animFrameReq: number;
    var start: number = null;

    pauseButton.onclick = ev => {
        if (pauseButton.value == "Resume") {
            pauseButton.value = "Pause";

            function step(timestamp: number) {
                if (!start) start = timestamp;
                const dt = timestamp - start;
                start = timestamp;

                renderer.advance(dt / 1000.0);
                renderer.render_canvas(canvas_context, ...getCurveViews());

                animFrameReq = requestAnimationFrame(step);
            }
            animFrameReq = requestAnimationFrame(step);
        } else if (pauseButton.value == "Pause") {
            pauseButton.value = "Resume";

            start = null;
            cancelAnimationFrame(animFrameReq);
        }
    };
    curveElems.forEach(elems => elems.forEach(
        elem => elem.onchange = ev => renderer.render_canvas(canvas_context, ...getCurveViews())
    ));

    document.getElementById("set_button").onclick = ev => {
        if (pauseButton.value === "Pause") {
            pauseButton.click();
        }
        renderer.reset(...getSetParams());
        renderer.render_canvas(canvas_context, ...getCurveViews());
    };

    document.getElementById("left_fn_select").onchange = ev => {
        renderer.left_ty = (ev.target as HTMLSelectElement).value;
    };
    document.getElementById("right_fn_select").onchange = ev => {
        renderer.right_ty = (ev.target as HTMLSelectElement).value;
    };
    document.getElementById("left_fn_input").onchange = ev => {
        const text = (ev.target as HTMLInputElement).value;
        renderer.left_func = eval("t => { return " + text + "; }");
    };
    document.getElementById("right_fn_input").onchange = ev => {
        const text = (ev.target as HTMLInputElement).value;
        renderer.right_func = eval("t => { return " + text + "; }");
    };
    ["bottom_u_input", "bottom_u_t_input", "a_input", "l_input"]
        .forEach(id => document.getElementById(id).onkeyup = submit_set);
}

function submit_set(ev: KeyboardEvent) {
    if (ev.key === 'Enter') {
        document.getElementById("set_button").click();
    }
}

main().catch(console.error)