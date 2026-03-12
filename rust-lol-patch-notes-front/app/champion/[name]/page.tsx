"use client";

import { useEffect, useState } from "react";
import { useParams, useSearchParams, useRouter } from "next/navigation";
import Image from "next/image";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Loader2, ArrowLeft } from "lucide-react";

interface AbilityChange {
  name: string;
  changes: string[];
}

interface ChampionNames {
  upper: string;
  lower: string;
}

interface PatchCard {
  champion_name: string;
  champion_image: ChampionNames;
  summary: string;
  context: string;
  abilities: AbilityChange[];
  patch_version: string;
}

export default function ChampionNotesPage() {
  const router = useRouter();
  const params = useParams();
  const searchParams = useSearchParams();

  const championName = params.name as string;
  const lang = searchParams.get("lang") || "pt-br";

  const [patchNotes, setPatchNotes] = useState<PatchCard[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    if (!championName) return;

    const fetchPatchNotes = async () => {
      try {
        const res = await fetch(
          `https://kzg6km3w-3111.brs.devtunnels.ms/champion_notes/?champion_name=${championName}&language=${lang}`,
        );

        if (!res.ok) {
          throw new Error(
            "Não foi possível carregar as notas de atualização deste campeão.",
          );
        }

        const data: PatchCard[] = await res.json();
        setPatchNotes(data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchPatchNotes();
  }, [championName, lang]);

  return (
    <div className="container mx-auto p-6 min-h-screen max-w-4xl">
      <Button
        type="button"
        className="mb-6 bg-teal-500 hover:bg-teal-600 text-white flex items-center gap-2"
        onClick={() => router.back()}
      >
        <ArrowLeft size={16} />
        Voltar
      </Button>

      {loading ? (
        <div className="flex flex-col items-center justify-center mt-32">
          <Loader2 className="h-12 w-12 animate-spin text-teal-500 mb-4" />
          <p className="text-muted-foreground">
            Buscando notas de atualização...
          </p>
        </div>
      ) : error ? (
        <div className="text-center mt-32">
          <h2 className="text-2xl font-bold text-teal-600 mb-2">Ops!</h2>
          <p className="text-muted-foreground">{error}</p>
        </div>
      ) : patchNotes.length === 0 ? (
        <div className="text-center mt-32">
          <p className="text-muted-foreground">
            Nenhuma nota de atualização encontrada para{" "}
            {decodeURIComponent(championName)}.
          </p>
        </div>
      ) : (
        <div className="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
          <div className="flex items-center gap-6 mb-8 bg-secondary/20 p-6 rounded-lg border border-teal-500/20">
            <div className="relative w-24 h-24 rounded-full overflow-hidden border-2 border-teal-500 shadow-lg">
              <Image
                fill
                src={patchNotes[0].champion_image.lower}
                alt={patchNotes[0].champion_name}
                className="object-cover"
                sizes="96px"
              />
            </div>
            <div>
              <h1 className="text-4xl font-bold capitalize text-foreground">
                {patchNotes[0].champion_name}
              </h1>
              <p className="text-muted-foreground">
                Histórico de Atualizações ({lang})
              </p>
            </div>
          </div>

          {patchNotes.map((note, index) => (
            <Card
              key={`${note.patch_version}-${index}`}
              className="border-l-4 border-l-teal-500 shadow-sm"
            >
              <CardHeader>
                <CardTitle className="text-2xl text-teal-600">
                  {note.patch_version}
                </CardTitle>
                {note.summary && (
                  <CardDescription className="text-base font-medium text-foreground mt-2">
                    {note.summary}
                  </CardDescription>
                )}
              </CardHeader>
              <CardContent className="space-y-6">
                {note.context && (
                  <blockquote className="border-l-2 border-teal-300 pl-4 italic text-muted-foreground bg-teal-500/5 p-3 rounded-r-md">
                    "{note.context}"
                  </blockquote>
                )}

                <div className="space-y-4">
                  {note.abilities.map((ability, idx) => (
                    <div key={idx} className="bg-secondary/10 p-4 rounded-md">
                      <h4 className="font-semibold text-lg mb-2 text-foreground flex items-center gap-2">
                        <span className="w-2 h-2 rounded-full bg-teal-400"></span>
                        {ability.name}
                      </h4>
                      <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
                        {ability.changes.map((change, cIdx) => (
                          <li key={cIdx} className="leading-relaxed">
                            {change}
                          </li>
                        ))}
                      </ul>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
