import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarCheckboxItem from '../components/ui/menubar/MenubarCheckboxItem.vue';

const meta = {
  title: 'MenubarCheckboxItem',
  component: MenubarCheckboxItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarCheckboxItem>;

export default meta;
type Story = StoryObj<typeof MenubarCheckboxItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};