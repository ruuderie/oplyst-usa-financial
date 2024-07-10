import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarSubContent from '../components/ui/menubar/MenubarSubContent.vue';

const meta = {
  title: 'MenubarSubContent',
  component: MenubarSubContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarSubContent>;

export default meta;
type Story = StoryObj<typeof MenubarSubContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};