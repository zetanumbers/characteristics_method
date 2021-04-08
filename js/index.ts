import { CurveView } from "../pkg/index";

async function main() {
  const {
    Renderer,
    UDiff,
    CurveView,
    ConstParams,
    CurveViews,
    VariableParams,
  } = await import("../pkg/index");

  // Enable interface
  document
    .querySelectorAll("input, select, button")
    .forEach((v: HTMLInputElement | HTMLSelectElement) => (v.disabled = false));

  const pauseButton = document.getElementById(
    "pause_button"
  ) as HTMLInputElement;
  const bottomUInput = document.getElementById(
    "bottom_u_input"
  ) as HTMLInputElement;
  const bottomUtInput = document.getElementById(
    "bottom_u_t_input"
  ) as HTMLInputElement;
  const aInput = document.getElementById("a_input") as HTMLInputElement;
  const lInput = document.getElementById("l_input") as HTMLInputElement;

  // get required params per set button click
  const getConstParams = () =>
    new ConstParams(
      eval("x => { return " + bottomUInput.value + "; }"),
      eval("x => { return " + bottomUtInput.value + "; }"),
      parseFloat(aInput.value),
      parseFloat(lInput.value)
    );

  const getSideFunc = (side: "left" | "right") => {
    const text = (document.getElementById(
      side + "_fn_input"
    ) as HTMLInputElement).value;
    const ty = (document.getElementById(
      side + "_fn_select"
    ) as HTMLInputElement).value;
    return new UDiff(ty, eval("(t) => { return " + text + "; }"));
  };
  const getVarParams = () =>
    new VariableParams(getSideFunc("left"), getSideFunc("right"));

  const getViewElems = (funcName: string) => {
    return [funcName + "_visible", funcName + "_color"].map((id) =>
      document.getElementById(id)
    ) as [HTMLInputElement, HTMLInputElement];
  };
  const curveElems = [
    getViewElems("u"),
    getViewElems("u_t"),
    getViewElems("u_x"),
  ];
  const getCurveViews = () =>
    new CurveViews(
      ...(curveElems.map(
        ([checkbox, colorInput]) =>
          new CurveView(checkbox.checked, colorInput.value)
      ) as [CurveView, CurveView, CurveView])
    );

  const renderer = new Renderer(
    getVarParams(),
    getConstParams(),
    getCurveViews()
  );

  const canvas_context = (document.getElementById(
    "canvas"
  ) as HTMLCanvasElement).getContext("2d");
  renderer.render_canvas(canvas_context);

  var animFrameReq: number;
  var start: number = null;

  pauseButton.onclick = (ev) => {
    if (pauseButton.value == "Resume") {
      pauseButton.value = "Pause";

      function step(timestamp: number) {
        if (!start) start = timestamp;
        const dt = timestamp - start;
        start = timestamp;

        renderer.advance(dt / 1000.0);
        renderer.render_canvas(canvas_context);

        animFrameReq = requestAnimationFrame(step);
      }
      animFrameReq = requestAnimationFrame(step);
    } else if (pauseButton.value == "Pause") {
      pauseButton.value = "Resume";

      start = null;
      cancelAnimationFrame(animFrameReq);
    }
  };

  function updateViews() {
    renderer.views = getCurveViews();
  }
  for (const [checkbox, colorInput] of curveElems) {
    checkbox.addEventListener("click", updateViews);
    colorInput.addEventListener("change", updateViews);
  }

  document.getElementById("set_button").addEventListener("click", (ev) => {
    if (pauseButton.value === "Pause") {
      pauseButton.click();
    }
    renderer.reset(getConstParams());
    renderer.render_canvas(canvas_context);
  });

  function updateVarParams() {
    renderer.var_params = getVarParams();
  }
  for (const id of [
    "left_fn_select",
    "right_fn_select",
    "left_fn_input",
    "right_fn_input",
  ]) {
    const element = document.getElementById(id);
    element.addEventListener("change", updateVarParams);
  }

  for (const id of [
    "bottom_u_input",
    "bottom_u_t_input",
    "a_input",
    "l_input",
  ]) {
    const element = document.getElementById(id);
    element.addEventListener("keyup", submit_set);
  }
}

function submit_set(ev: KeyboardEvent) {
  if (ev.key === "Enter") {
    document.getElementById("set_button").click();
  }
}

main().catch(console.error);
