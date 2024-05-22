export type ServerModel = {
	name: string,
	url: string
}

export type ChangeServerResult = {
    current: ServerModel,
    list: ServerModel[],
}
