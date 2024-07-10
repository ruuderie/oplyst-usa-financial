import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarContent from '../components/ui/menubar/MenubarContent.vue';

const meta = {
  title: 'MenubarContent',
  component: MenubarContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarContent>;

export default meta;
type Story = StoryObj<typeof MenubarContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};