"use client";

// import { Character } from "@/interfaces/character";
import personagens from "@/consts/personagens";
import { useParams, useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";

export default function About() {
  const { slug } = useParams();
  const router = useRouter();
  const name = slug;
  console.log(slug);
  const personagemAtual = personagens.find(
    (personagem) => personagem.name === name,
  );
  return (
    <div>
      <h1>{personagemAtual?.name}</h1>
      <h1>{personagemAtual?.description}</h1>
      <Button
        type="button"
        className="bg-red-400 min-w-24 min-h-1/3 "
        onClick={() => router.back()}
      >        voltar
      </Button>
    </div>
  );
}
