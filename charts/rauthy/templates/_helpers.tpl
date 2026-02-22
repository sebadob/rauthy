{{/*
Expand the name of the chart.
*/}}
{{- define "rauthy.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "rauthy.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "rauthy.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "rauthy.labels" -}}
helm.sh/chart: {{ include "rauthy.chart" . }}
{{ include "rauthy.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Config secret name
*/}}
{{- define "rauthy.configSecretName" -}}
{{- if .Values.configSecretNameOverride }}
{{ .Values.configSecretNameOverride }}
{{- else }}
{{- default (include "rauthy.fullname" .) .Values.serviceAccount.name }}-config
{{- end }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "rauthy.selectorLabels" -}}
app.kubernetes.io/name: {{ include "rauthy.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "rauthy.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "rauthy.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}
