import { Head } from "$fresh/runtime.ts";
import Kiosk from "../islands/kiosk.tsx";

export default function Home() {
  return (
    <>
      <Head>
        <title>kiosk</title>
      </Head>
      <div class="container mx-auto grid grid-cols-1 gap-2 md:grid-cols-2">
        <div class="mx-auto h-screen">
          <Kiosk />
        </div>
        <div class="mx-auto h-screen">
          this area is order state
        </div>
      </div>
    </>
  );
}
