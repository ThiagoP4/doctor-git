class DoctorGit < Formula
  desc "Auto Gerador de Conquistas Gamificadas para Git baseado em Inteligência Artificial"
  homepage "https://github.com/ThiagoP4/doctor-git"
  url "https://github.com/ThiagoP4/doctor-git/releases/download/v0.1.0/doctor-git-mac.tar.gz"
  version "0.1.0"
  
  # TODO: Quando exportar a release v0.1.0, o Homebrew exigirá a hash de segurança sha256. 
  # Basta pegar a hash do terminal e colar aqui em baixo:
  # sha256 "..."

  def install
    bin.install "doctor-git"
  end

  def caveats
    <<~EOS
      -------------------------------------------------------
      [Doctor Git] Instalado Globalmente com Sucesso! \u2705
      -------------------------------------------------------
      Para vigiar qualquer um de seus projetos Git:
        1. Entre na pasta local do projeto
        2. Digite o comando: doctor-git init
      -------------------------------------------------------
    EOS
  end
end
