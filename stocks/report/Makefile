.PHONY: clean format lint lint-html
.DEFAULT_GOAL := report

format: clean
	latexindent -w report.tex > /dev/null
clean:
	rm -rf output.html *.aux *.out *.fls *.fdb_latexmk *.log *.gz *.bak.* _minted-report *.bak* *.zip *.toc
report:
	pdflatex -shell-escape report
lint: report
	textidote --check en_CA report.tex
lint-html: report
	-textidote --output html --check en_CA report.tex > output.html
	-xdg-open ./output.html

#submission:
#	-zip -r lab1.zip report.pdf
